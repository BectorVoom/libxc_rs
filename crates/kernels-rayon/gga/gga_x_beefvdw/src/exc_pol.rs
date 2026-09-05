//! GGA_X_BEEFVDW exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_beefvdw.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_beefvdw_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = t18 + f64x8::splat(1.0);
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t39 = v_sigma0 * t38;
            let t42 = f64x8::splat(4.0) + t33 * t39 / f64x8::splat(24.0);
            let t43 = f64x8::splat(1.0) / t42;
            let t45 = t33 * t39 * t43;
            let t47 = t45 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t48 = t47 * t47;
            let t49 = t48 * t48;
            let t51 = t48 * t47;
            let t54 = t49 * t49;
            let t55 = t54 * t54;
            let t57 = t49 * t47;
            let t58 = t54 * t57;
            let t60 = t49 * t48;
            let t61 = t54 * t60;
            let t63 = t54 * t51;
            let t65 = t54 * t49;
            let t68 = t49 * t51;
            let t71 = t55 * t49;
            let t73 = t55 * t57;
            let t75 = t55 * t60;
            let t77 = t55 * t54;
            let t79 = -f64x8::splat(0.6945973517763898) * t49 + f64x8::splat(0.527556201155898) * t51 - f64x8::splat(0.38916037779196816) * t48 - f64x8::splat(168370.8413901412) * t55 - f64x8::splat(2810.240180568463) * t58 + f64x8::splat(70504.54186903402) * t61 + f64x8::splat(2274.8997850816486) * t63 - f64x8::splat(20148.24517562505) * t65 - f64x8::splat(442.33229018433804) * t54 + f64x8::splat(86.00573049927964) * t68 + f64x8::splat(30.54203495931585) * t60 - f64x8::splat(323524.0313604933) * t71 + f64x8::splat(180782.00670879145) * t73 + f64x8::splat(255894.79526235335) * t75 - f64x8::splat(132044.6618218215) * t77;
            let t80 = t55 * t68;
            let t82 = t54 * t47;
            let t83 = t55 * t82;
            let t85 = t55 * t65;
            let t87 = t54 * t48;
            let t88 = t55 * t87;
            let t90 = t55 * t63;
            let t94 = t55 * t51;
            let t96 = t55 * t47;
            let t98 = t55 * t48;
            let t104 = t54 * t68;
            let t106 = f64x8::splat(1.1313514630621233) - f64x8::splat(161142.1539984628) * t80 + f64x8::splat(90365.6111085228) * t83 - f64x8::splat(5427.777462637186) * t85 + f64x8::splat(40074.93585443239) * t88 - f64x8::splat(29150.193011493262) * t90 + f64x8::splat(4135.586188014654) * t55 * t58 - f64x8::splat(129814.81812794984) * t94 + f64x8::splat(56174.00797937267) * t96 + f64x8::splat(279670.48856303055) * t98 + f64x8::splat(3783.53964072524) * t87 - f64x8::splat(7.2975787893717134) * t57 - f64x8::splat(617.547861045286) * t82 + f64x8::splat(0.037534251004296526) * t45 - f64x8::splat(10276.426607863825) * t104;
            let t107 = t79 + t106;
            let t111 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t107));
            let t112 = (v_rho1).simd_le(dens_threshold);
            let t113 = -t16;
            let t115 = ((t14).select(t11, (t10).select(t15, t113 * t7)));
            let t116 = t115 + f64x8::splat(1.0);
            let t117 = (t116).simd_le(zeta_threshold);
            let t118 = (simd::cbrt(t116));
            let t120 = ((t117).select(t22, t118 * t116));
            let t121 = t120 * t26;
            let t122 = v_rho1 * v_rho1;
            let t123 = (simd::cbrt(v_rho1));
            let t124 = t123 * t123;
            let t126 = f64x8::splat(1.0) / t124 / t122;
            let t127 = v_sigma2 * t126;
            let t130 = f64x8::splat(4.0) + t33 * t127 / f64x8::splat(24.0);
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = t33 * t127 * t131;
            let t135 = t133 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t136 = t135 * t135;
            let t137 = t136 * t135;
            let t138 = t136 * t136;
            let t139 = t138 * t137;
            let t140 = t138 * t138;
            let t141 = t140 * t139;
            let t143 = t140 * t138;
            let t145 = t138 * t135;
            let t146 = t140 * t145;
            let t148 = t140 * t137;
            let t152 = t140 * t135;
            let t155 = t140 * t140;
            let t156 = t155 * t148;
            let t160 = t138 * t136;
            let t163 = t155 * t140;
            let t165 = t155 * t152;
            let t167 = -f64x8::splat(10276.426607863825) * t141 - f64x8::splat(20148.24517562505) * t143 - f64x8::splat(2810.240180568463) * t146 + f64x8::splat(2274.8997850816486) * t148 + f64x8::splat(86.00573049927964) * t139 - f64x8::splat(442.33229018433804) * t140 - f64x8::splat(617.547861045286) * t152 + f64x8::splat(0.527556201155898) * t137 - f64x8::splat(29150.193011493262) * t156 - f64x8::splat(0.38916037779196816) * t136 - f64x8::splat(7.2975787893717134) * t145 + f64x8::splat(30.54203495931585) * t160 - f64x8::splat(0.6945973517763898) * t138 - f64x8::splat(132044.6618218215) * t163 + f64x8::splat(90365.6111085228) * t165;
            let t169 = t155 * t135;
            let t171 = t140 * t160;
            let t175 = t155 * t143;
            let t177 = t155 * t139;
            let t179 = t155 * t145;
            let t181 = t155 * t160;
            let t183 = t155 * t138;
            let t185 = t155 * t136;
            let t187 = t155 * t137;
            let t189 = t140 * t136;
            let t192 = t155 * t189;
            let t194 = f64x8::splat(1.1313514630621233) - f64x8::splat(168370.8413901412) * t155 + f64x8::splat(56174.00797937267) * t169 + f64x8::splat(70504.54186903402) * t171 + f64x8::splat(4135.586188014654) * t155 * t146 - f64x8::splat(5427.777462637186) * t175 - f64x8::splat(161142.1539984628) * t177 + f64x8::splat(180782.00670879145) * t179 + f64x8::splat(255894.79526235335) * t181 - f64x8::splat(323524.0313604933) * t183 + f64x8::splat(279670.48856303055) * t185 - f64x8::splat(129814.81812794984) * t187 + f64x8::splat(3783.53964072524) * t189 + f64x8::splat(0.037534251004296526) * t133 + f64x8::splat(40074.93585443239) * t192;
            let t195 = t167 + t194;
            let t199 = ((t112).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t121 * t195));
            let tzk0 = t111 + t199;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
