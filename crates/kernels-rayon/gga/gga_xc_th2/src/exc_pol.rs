//! GGA_XC_TH2 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th2.c`
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
pub fn gga_xc_th2_exc_pol(
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
            let t1 = (simd::pow(v_rho0, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t4 = (simd::pow(v_rho1, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t7 = (simd::pow(v_rho0, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t8 = t7 * v_rho0;
            let t10 = (simd::pow(v_rho1, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t11 = t10 * v_rho1;
            let t13 = (simd::cbrt(v_rho0));
            let t14 = t13 * v_rho0;
            let t16 = (simd::cbrt(v_rho1));
            let t17 = t16 * v_rho1;
            let t19 = ((v_rho0).sqrt());
            let t20 = t19 * v_rho0;
            let t22 = ((v_rho1).sqrt());
            let t23 = t22 * v_rho1;
            let t25 = t13 * t13;
            let t26 = t25 * v_rho0;
            let t28 = t16 * t16;
            let t29 = t28 * v_rho1;
            let t31 = t1 * t1;
            let t32 = t31 * t31;
            let t33 = t32 * t1;
            let t35 = t4 * t4;
            let t36 = t35 * t35;
            let t37 = t36 * t4;
            let t39 = v_rho0 * t33 + v_rho1 * t37;
            let t40 = ((v_sigma0).sqrt());
            let t41 = f64x8::splat(1.0) / t14;
            let t42 = t40 * t41;
            let t43 = v_rho0 - v_rho1;
            let t44 = v_rho0 + v_rho1;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t43 * t45;
            let t47 = f64x8::splat(1.0) + t46;
            let t48 = (t47).simd_le(zeta_threshold);
            let t49 = (simd::cbrt(zeta_threshold));
            let t50 = t49 * zeta_threshold;
            let t51 = (simd::cbrt(t47));
            let t53 = ((t48).select(t50, t51 * t47));
            let t54 = f64x8::splat(M_CBRT2);
            let t55 = t54 * t54;
            let t56 = t53 * t55;
            let t58 = ((v_sigma2).sqrt());
            let t59 = f64x8::splat(1.0) / t17;
            let t60 = t58 * t59;
            let t61 = f64x8::splat(1.0) - t46;
            let t62 = (t61).simd_le(zeta_threshold);
            let t63 = (simd::cbrt(t61));
            let t65 = ((t62).select(t50, t63 * t61));
            let t66 = t65 * t55;
            let t69 = t42 * t56 / f64x8::splat(4.0) + t60 * t66 / f64x8::splat(4.0);
            let t72 = t20 + t23;
            let t75 = f64x8::splat(0.678831) * t1 * v_rho0 + f64x8::splat(0.678831) * t4 * v_rho1 - f64x8::splat(1.75821) * t8 - f64x8::splat(1.75821) * t11 + f64x8::splat(1.27676) * t14 + f64x8::splat(1.27676) * t17 - f64x8::splat(1.60789) * t20 - f64x8::splat(1.60789) * t23 + f64x8::splat(0.36561) * t26 + f64x8::splat(0.36561) * t29 - f64x8::splat(0.0906635) * t39 * t69 + f64x8::splat(0.0734865) * t72 * t69;
            let t76 = t26 + t29;
            let t79 = t7 * t7;
            let t80 = t79 * t79;
            let t81 = t80 * t7;
            let t82 = t81 * v_rho0;
            let t83 = t10 * t10;
            let t84 = t83 * t83;
            let t85 = t84 * t10;
            let t86 = t85 * v_rho1;
            let t87 = t82 + t86;
            let t90 = v_rho0 * v_rho0;
            let t92 = f64x8::splat(1.0) / t25 / t90;
            let t93 = v_sigma0 * t92;
            let t94 = t53 * t53;
            let t95 = t94 * t54;
            let t96 = t93 * t95;
            let t97 = v_rho1 * v_rho1;
            let t99 = f64x8::splat(1.0) / t28 / t97;
            let t100 = v_sigma2 * t99;
            let t101 = t65 * t65;
            let t102 = t101 * t54;
            let t103 = t100 * t102;
            let t105 = t96 / f64x8::splat(8.0) + t103 / f64x8::splat(8.0);
            let t110 = t90 + t97;
            let t116 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t117 = t44 * t44;
            let t118 = (simd::cbrt(t44));
            let t119 = t118 * t118;
            let t121 = f64x8::splat(1.0) / t119 / t117;
            let t122 = t116 * t121;
            let t123 = t96 / f64x8::splat(4.0) + t103 / f64x8::splat(4.0) - t122;
            let t130 = t8 + t11;
            let t131 = t43 * t43;
            let t132 = t130 * t131;
            let t133 = f64x8::splat(1.0) / t117;
            let t136 = t14 + t17;
            let t137 = t136 * t131;
            let t140 = t72 * t131;
            let t143 = (simd::pow_5_3(v_rho0));
            let t144 = (simd::pow_5_3(v_rho1));
            let t145 = t143 + t144;
            let t146 = t145 * t131;
            let t149 = f64x8::splat(0.0735705) * t76 * t69 - f64x8::splat(0.03584585) * t87 * t69 - f64x8::splat(0.02035835) * t76 * t105 + f64x8::splat(0.01073125) * t87 * t105 - f64x8::splat(0.000384078) * t110 * t105 + f64x8::splat(0.0310377) * t76 * t123 - f64x8::splat(0.0720326) * t87 * t123 + f64x8::splat(0.0446562) * t110 * t123 - f64x8::splat(0.266802) * t132 * t133 + f64x8::splat(1.50822) * t137 * t133 - f64x8::splat(1.94515) * t140 * t133 + f64x8::splat(0.679078) * t146 * t133;
            let tzk0 = (t75 + t149) * t45;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
