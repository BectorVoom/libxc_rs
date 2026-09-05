//! GGA_X_SFAT exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat.c`
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
pub fn gga_x_sfat_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t5 = f64x8::splat(1.0) / t3 * t2;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * t7 * v_rho0).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * t7 * v_rho1).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t7 * t16)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = t25 * t5;
            let t27 = (simd::cbrt(t6));
            let t28 = t2 * t2;
            let t29 = t28 * f64x8::splat(M_PI);
            let t30 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = t33 * t32;
            let t35 = t32 * t28;
            let t36 = t33 * t35;
            let t37 = v_rho0 * v_rho0;
            let t38 = (simd::cbrt(v_rho0));
            let t39 = t38 * t38;
            let t41 = f64x8::splat(1.0) / t39 / t37;
            let t42 = t41 * v_sigma0;
            let t43 = ((v_sigma0).sqrt());
            let t45 = f64x8::splat(1.0) / t38 / v_rho0;
            let t46 = t45 * t43;
            let t47 = (simd::ln(t46 + ((t46 * t46 + f64x8::splat(1.0)).sqrt())));
            let t50 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t47 * t46;
            let t51 = f64x8::splat(1.0) / t50;
            let t55 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t51 * t42 * t36;
            let t58 = f64x8::splat(1.0) / t55 * t34 * t29;
            let t59 = ((t58).sqrt());
            let t61 = f64x8::splat(1.0) / t59 * param_hyb_omega_0;
            let t62 = f64x8::splat(M_CBRT2);
            let t63 = t6 * t19;
            let t64 = (simd::cbrt(t63));
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t65 * t62;
            let t68 = t66 * t61 / f64x8::splat(2.0);
            let t69 = (f64x8::splat(1.92)).simd_le(t68);
            let t70 = (f64x8::splat(1.92)).simd_lt(t68);
            let t71 = ((t70).select(t68, f64x8::splat(1.92)));
            let t72 = t71 * t71;
            let t73 = t72 * t72;
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = t73 * t72;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t73 * t73;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t79 * t72;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t79 * t73;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = t79 * t76;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = t79 * t79;
            let t92 = f64x8::splat(1.0) / t91;
            let t95 = f64x8::splat(1.0) / t91 / t72;
            let t98 = f64x8::splat(1.0) / t91 / t73;
            let t101 = f64x8::splat(1.0) / t91 / t76;
            let t104 = f64x8::splat(1.0) / t91 / t79;
            let t107 = f64x8::splat(1.0) / t91 / t82;
            let t110 = f64x8::splat(1.0) / t91 / t85;
            let t113 = f64x8::splat(1.0) / t91 / t88;
            let t115 = t91 * t91;
            let t116 = f64x8::splat(1.0) / t115;
            let t119 = f64x8::splat(1.0) / t115 / t72;
            let t122 = f64x8::splat(1.0) / t115 / t73;
            let t126 = -t74 / f64x8::splat(30.0) + t77 / f64x8::splat(70.0) - t80 / f64x8::splat(135.0) + t83 / f64x8::splat(231.0) - t86 / f64x8::splat(364.0) + t89 / f64x8::splat(540.0) - t92 / f64x8::splat(765.0) + t95 / f64x8::splat(1045.0) - t98 / f64x8::splat(1386.0) + t101 / f64x8::splat(1794.0) - t104 / f64x8::splat(2275.0) + t107 / f64x8::splat(2835.0) - t110 / f64x8::splat(3480.0) + t113 / f64x8::splat(4216.0) - t116 / f64x8::splat(5049.0) + t119 / f64x8::splat(5985.0) - t122 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t72 / f64x8::splat(9.0);
            let t127 = ((t70).select(f64x8::splat(1.92), t68));
            let t128 = (simd::atan2(f64x8::splat(1.0), t127));
            let t129 = t127 * t127;
            let t130 = t129 + f64x8::splat(3.0);
            let t131 = f64x8::splat(1.0) / t129;
            let t132 = f64x8::splat(1.0) + t131;
            let t133 = (simd::ln(t132));
            let t135 = -t133 * t130 + f64x8::splat(1.0);
            let t138 = t128 + t135 * t127 / f64x8::splat(4.0);
            let t142 = ((t69).select(t126, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t138 * t127));
            let t143 = t142 * t27;
            let t144 = t55 * t143;
            let t147 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t144 * t26));
            let t148 = (v_rho1).simd_le(dens_threshold);
            let t149 = -t16;
            let t151 = ((t14).select(t11, (t10).select(t15, t7 * t149)));
            let t152 = f64x8::splat(1.0) + t151;
            let t153 = (t152).simd_le(zeta_threshold);
            let t154 = (simd::cbrt(t152));
            let t156 = ((t153).select(t22, t154 * t152));
            let t157 = t156 * t5;
            let t158 = v_rho1 * v_rho1;
            let t159 = (simd::cbrt(v_rho1));
            let t160 = t159 * t159;
            let t162 = f64x8::splat(1.0) / t160 / t158;
            let t163 = t162 * v_sigma2;
            let t164 = ((v_sigma2).sqrt());
            let t166 = f64x8::splat(1.0) / t159 / v_rho1;
            let t167 = t166 * t164;
            let t168 = (simd::ln(t167 + ((t167 * t167 + f64x8::splat(1.0)).sqrt())));
            let t171 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t168 * t167;
            let t172 = f64x8::splat(1.0) / t171;
            let t176 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t172 * t163 * t36;
            let t179 = f64x8::splat(1.0) / t176 * t34 * t29;
            let t180 = ((t179).sqrt());
            let t182 = f64x8::splat(1.0) / t180 * param_hyb_omega_0;
            let t183 = t6 * t152;
            let t184 = (simd::cbrt(t183));
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t185 * t62;
            let t188 = t186 * t182 / f64x8::splat(2.0);
            let t189 = (f64x8::splat(1.92)).simd_le(t188);
            let t190 = (f64x8::splat(1.92)).simd_lt(t188);
            let t191 = ((t190).select(t188, f64x8::splat(1.92)));
            let t192 = t191 * t191;
            let t193 = t192 * t192;
            let t194 = f64x8::splat(1.0) / t193;
            let t196 = t193 * t192;
            let t197 = f64x8::splat(1.0) / t196;
            let t199 = t193 * t193;
            let t200 = f64x8::splat(1.0) / t199;
            let t202 = t199 * t192;
            let t203 = f64x8::splat(1.0) / t202;
            let t205 = t199 * t193;
            let t206 = f64x8::splat(1.0) / t205;
            let t208 = t199 * t196;
            let t209 = f64x8::splat(1.0) / t208;
            let t211 = t199 * t199;
            let t212 = f64x8::splat(1.0) / t211;
            let t215 = f64x8::splat(1.0) / t211 / t192;
            let t218 = f64x8::splat(1.0) / t211 / t193;
            let t221 = f64x8::splat(1.0) / t211 / t196;
            let t224 = f64x8::splat(1.0) / t211 / t199;
            let t227 = f64x8::splat(1.0) / t211 / t202;
            let t230 = f64x8::splat(1.0) / t211 / t205;
            let t233 = f64x8::splat(1.0) / t211 / t208;
            let t235 = t211 * t211;
            let t236 = f64x8::splat(1.0) / t235;
            let t239 = f64x8::splat(1.0) / t235 / t192;
            let t242 = f64x8::splat(1.0) / t235 / t193;
            let t246 = -t194 / f64x8::splat(30.0) + t197 / f64x8::splat(70.0) - t200 / f64x8::splat(135.0) + t203 / f64x8::splat(231.0) - t206 / f64x8::splat(364.0) + t209 / f64x8::splat(540.0) - t212 / f64x8::splat(765.0) + t215 / f64x8::splat(1045.0) - t218 / f64x8::splat(1386.0) + t221 / f64x8::splat(1794.0) - t224 / f64x8::splat(2275.0) + t227 / f64x8::splat(2835.0) - t230 / f64x8::splat(3480.0) + t233 / f64x8::splat(4216.0) - t236 / f64x8::splat(5049.0) + t239 / f64x8::splat(5985.0) - t242 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t192 / f64x8::splat(9.0);
            let t247 = ((t190).select(f64x8::splat(1.92), t188));
            let t248 = (simd::atan2(f64x8::splat(1.0), t247));
            let t249 = t247 * t247;
            let t250 = t249 + f64x8::splat(3.0);
            let t251 = f64x8::splat(1.0) / t249;
            let t252 = f64x8::splat(1.0) + t251;
            let t253 = (simd::ln(t252));
            let t255 = -t253 * t250 + f64x8::splat(1.0);
            let t258 = t248 + t255 * t247 / f64x8::splat(4.0);
            let t262 = ((t189).select(t246, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t247));
            let t263 = t262 * t27;
            let t264 = t176 * t263;
            let t267 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t264 * t157));
            let tzk0 = t147 + t267;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
