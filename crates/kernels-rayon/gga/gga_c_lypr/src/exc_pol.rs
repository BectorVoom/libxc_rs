//! GGA_C_LYPR exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lypr.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_lypr_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_m1: f64,
    param_omega: f64,
    param_d: f64,
    param_m2: f64,
    param_b: f64,
    param_c: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_m1 = f64x8::splat(param_m1);
    let param_omega = f64x8::splat(param_omega);
    let param_d = f64x8::splat(param_d);
    let param_m2 = f64x8::splat(param_m2);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_a = f64x8::splat(param_a);
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
            let t1 = param_m1 * param_omega;
            let t2 = v_rho0 + v_rho1;
            let t3 = (simd::cbrt(t2));
            let t4 = f64x8::splat(1.0) / t3;
            let t6 = (simd::erfc(t1 * t4));
            let t7 = v_rho0 - v_rho1;
            let t8 = t7 * t7;
            let t9 = t2 * t2;
            let t10 = f64x8::splat(1.0) / t9;
            let t12 = -t8 * t10 + f64x8::splat(1.0);
            let t13 = t6 * t12;
            let t15 = param_d * t4 + f64x8::splat(1.0);
            let t16 = f64x8::splat(1.0) / t15;
            let t18 = param_m2 * param_omega;
            let t20 = (simd::erfc(t18 * t4));
            let t21 = t20 * param_b;
            let t23 = (simd::exp(-param_c * t4));
            let t24 = t23 * t16;
            let t26 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t27 = t3 * t3;
            let t29 = f64x8::splat(1.0) / t27 / t9;
            let t30 = t26 * t29;
            let t32 = param_d * t16 + param_c;
            let t33 = t32 * t4;
            let t35 = f64x8::splat(47.0) - f64x8::splat(7.0) * t33;
            let t38 = t12 * t35 / f64x8::splat(72.0) - f64x8::splat(2.0) / f64x8::splat(3.0);
            let t40 = f64x8::splat(M_CBRT3);
            let t41 = t40 * t40;
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = t41 * t44;
            let t46 = f64x8::splat(1.0) / t2;
            let t47 = t7 * t46;
            let t48 = f64x8::splat(1.0) + t47;
            let t49 = (t48).simd_le(zeta_threshold);
            let t50 = zeta_threshold * zeta_threshold;
            let t51 = (simd::cbrt(zeta_threshold));
            let t52 = t51 * t51;
            let t53 = t52 * t50;
            let t54 = t48 * t48;
            let t55 = (simd::cbrt(t48));
            let t56 = t55 * t55;
            let t57 = t56 * t54;
            let t58 = ((t49).select(t53, t57));
            let t59 = f64x8::splat(1.0) - t47;
            let t60 = (t59).simd_le(zeta_threshold);
            let t61 = t59 * t59;
            let t62 = (simd::cbrt(t59));
            let t63 = t62 * t62;
            let t64 = t63 * t61;
            let t65 = ((t60).select(t53, t64));
            let t66 = t58 + t65;
            let t70 = f64x8::splat(M_CBRT2);
            let t71 = t70 * t12;
            let t73 = f64x8::splat(5.0) / f64x8::splat(2.0) - t33 / f64x8::splat(18.0);
            let t74 = v_rho0 * v_rho0;
            let t75 = (simd::cbrt(v_rho0));
            let t76 = t75 * t75;
            let t78 = f64x8::splat(1.0) / t76 / t74;
            let t79 = v_sigma0 * t78;
            let t80 = t79 * t58;
            let t81 = v_rho1 * v_rho1;
            let t82 = (simd::cbrt(v_rho1));
            let t83 = t82 * t82;
            let t85 = f64x8::splat(1.0) / t83 / t81;
            let t86 = v_sigma2 * t85;
            let t87 = t86 * t65;
            let t88 = t80 + t87;
            let t89 = t73 * t88;
            let t92 = t33 - f64x8::splat(11.0);
            let t94 = t52 * t50 * zeta_threshold;
            let t97 = ((t49).select(t94, t56 * t54 * t48));
            let t101 = ((t60).select(t94, t63 * t61 * t59));
            let t103 = t86 * t101 + t79 * t97;
            let t104 = t92 * t103;
            let t109 = ((t49).select(t50, t54));
            let t110 = t109 * v_sigma2;
            let t111 = t85 * t65;
            let t114 = ((t60).select(t50, t61));
            let t115 = t114 * v_sigma0;
            let t116 = t78 * t58;
            let t122 = -t30 * t38 - f64x8::splat(3.0) / f64x8::splat(20.0) * t45 * t12 * t66 + t71 * t89 / f64x8::splat(32.0) + t71 * t104 / f64x8::splat(576.0) - t70 * (f64x8::splat(2.0) / f64x8::splat(3.0) * t80 + f64x8::splat(2.0) / f64x8::splat(3.0) * t87 - t110 * t111 / f64x8::splat(4.0) - t115 * t116 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t123 = t24 * t122;
            let t125 = param_b * t23;
            let t126 = ((f64x8::splat(M_PI)).sqrt());
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t16 * t127;
            let t130 = t125 * t128 * param_m2;
            let t131 = param_m2 * param_m2;
            let t132 = param_omega * param_omega;
            let t134 = f64x8::splat(1.0) / t27;
            let t136 = (simd::exp(-t131 * t132 * t134));
            let t137 = param_omega * t136;
            let t138 = t4 * t12;
            let t142 = t47 / f64x8::splat(6.0);
            let t143 = f64x8::splat(7.0) / f64x8::splat(6.0) + t142;
            let t144 = t143 * v_sigma0;
            let t145 = t70 * t78;
            let t146 = t145 * t58;
            let t149 = f64x8::splat(7.0) / f64x8::splat(6.0) - t142;
            let t150 = t149 * v_sigma2;
            let t151 = t70 * t85;
            let t152 = t151 * t65;
            let t155 = f64x8::splat(7.0) / f64x8::splat(6.0) * t30 - f64x8::splat(7.0) / f64x8::splat(48.0) * t70 * t88 + t144 * t146 / f64x8::splat(8.0) + t150 * t152 / f64x8::splat(8.0);
            let tzk0 = param_a * (-t13 * t16 + t21 * t123 + t130 * t137 * t138 * t155 / f64x8::splat(6.0));
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
