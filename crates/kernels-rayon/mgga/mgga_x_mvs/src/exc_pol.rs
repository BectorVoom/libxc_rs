//! MGGA_X_MVS exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvs.c`
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
pub fn mgga_x_mvs_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_c1 = f64x8::splat(param_c1);
    let param_e1 = f64x8::splat(param_e1);
    let param_k0 = f64x8::splat(param_k0);
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = (simd::cbrt(v_rho0));
            let t30 = t29 * t29;
            let t32 = f64x8::splat(1.0) / t30 / v_rho0;
            let t34 = v_rho0 * v_rho0;
            let t36 = f64x8::splat(1.0) / t30 / t34;
            let t39 = v_tau0 * t32 - v_sigma0 * t36 / f64x8::splat(8.0);
            let t40 = f64x8::splat(M_CBRT6);
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t49 = param_k0 * (f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t39 * t40 * t45);
            let t50 = t39 * t39;
            let t52 = t40 * t40;
            let t54 = f64x8::splat(1.0) / t43 / t42;
            let t55 = t52 * t54;
            let t58 = f64x8::splat(1.0) + f64x8::splat(25.0) / f64x8::splat(81.0) * param_e1 * t50 * t55;
            let t59 = t58 * t58;
            let t60 = t50 * t50;
            let t62 = t42 * t42;
            let t64 = f64x8::splat(1.0) / t44 / t62;
            let t65 = t40 * t64;
            let t68 = t59 + f64x8::splat(1250.0) / f64x8::splat(2187.0) * param_c1 * t60 * t65;
            let t69 = ((t68).sqrt().sqrt());
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = t49 * t70 + f64x8::splat(1.0);
            let t74 = param_b * t52;
            let t75 = v_sigma0 * v_sigma0;
            let t76 = t54 * t75;
            let t77 = t34 * t34;
            let t78 = t77 * v_rho0;
            let t80 = f64x8::splat(1.0) / t29 / t78;
            let t84 = f64x8::splat(1.0) + t74 * t76 * t80 / f64x8::splat(576.0);
            let t85 = (simd::pow(t84, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t86 = f64x8::splat(1.0) / t85;
            let t87 = t28 * t72 * t86;
            let t90 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t87));
            let t91 = (v_rho1).simd_le(dens_threshold);
            let t92 = -t17;
            let t94 = ((t15).select(t12, (t11).select(t16, t92 * t8)));
            let t95 = f64x8::splat(1.0) + t94;
            let t96 = (t95).simd_le(zeta_threshold);
            let t97 = (simd::cbrt(t95));
            let t99 = ((t96).select(t23, t97 * t95));
            let t100 = t6 * t99;
            let t101 = (simd::cbrt(v_rho1));
            let t102 = t101 * t101;
            let t104 = f64x8::splat(1.0) / t102 / v_rho1;
            let t106 = v_rho1 * v_rho1;
            let t108 = f64x8::splat(1.0) / t102 / t106;
            let t111 = v_tau1 * t104 - v_sigma2 * t108 / f64x8::splat(8.0);
            let t116 = param_k0 * (f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t111 * t40 * t45);
            let t117 = t111 * t111;
            let t121 = f64x8::splat(1.0) + f64x8::splat(25.0) / f64x8::splat(81.0) * param_e1 * t117 * t55;
            let t122 = t121 * t121;
            let t123 = t117 * t117;
            let t127 = t122 + f64x8::splat(1250.0) / f64x8::splat(2187.0) * param_c1 * t123 * t65;
            let t128 = ((t127).sqrt().sqrt());
            let t129 = f64x8::splat(1.0) / t128;
            let t131 = t116 * t129 + f64x8::splat(1.0);
            let t133 = v_sigma2 * v_sigma2;
            let t134 = t54 * t133;
            let t135 = t106 * t106;
            let t136 = t135 * v_rho1;
            let t138 = f64x8::splat(1.0) / t101 / t136;
            let t142 = f64x8::splat(1.0) + t74 * t134 * t138 / f64x8::splat(576.0);
            let t143 = (simd::pow(t142, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t28 * t131 * t144;
            let t148 = ((t91).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t100 * t145));
            let tzk0 = t90 + t148;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
