//! MGGA_X_LTA vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_lta.c`
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
pub fn mgga_x_lta_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_ltafrac: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_ltafrac = f64x8::splat(param_ltafrac);
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
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
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = (simd::cbrt(v_rho0));
            let t30 = t29 * t29;
            let t34 = f64x8::splat(M_CBRT6);
            let t35 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t36 = (simd::cbrt(t35));
            let t37 = t36 * t36;
            let t39 = t34 / t37;
            let t42 = f64x8::splat(4.0) / f64x8::splat(5.0) * param_ltafrac;
            let t43 = (simd::pow(f64x8::splat(5.0) / f64x8::splat(9.0) * v_tau0 / t30 / v_rho0 * t39, t42));
            let t47 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t43));
            let t48 = (v_rho1).simd_le(dens_threshold);
            let t49 = -t17;
            let t51 = ((t15).select(t12, (t11).select(t16, t49 * t8)));
            let t52 = f64x8::splat(1.0) + t51;
            let t53 = (t52).simd_le(zeta_threshold);
            let t54 = (simd::cbrt(t52));
            let t56 = ((t53).select(t23, t54 * t52));
            let t57 = t56 * t27;
            let t58 = (simd::cbrt(v_rho1));
            let t59 = t58 * t58;
            let t65 = (simd::pow(f64x8::splat(5.0) / f64x8::splat(9.0) * v_tau1 / t59 / v_rho1 * t39, t42));
            let t69 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t57 * t65));
            let tzk0 = t47 + t69;
            acc_zk = tzk0;
            let t70 = t7 * t7;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t17 * t71;
            let t74 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t72)));
            let t77 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t74));
            let t78 = t77 * t27;
            let t82 = t27 * t27;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t26 * t83;
            let t87 = t6 * t84 * t43 / f64x8::splat(8.0);
            let t88 = t6 * t26;
            let t89 = t27 * t43;
            let t90 = f64x8::splat(1.0) / v_rho0;
            let t91 = param_ltafrac * t90;
            let t92 = t89 * t91;
            let t96 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t78 * t43 - t87 + t88 * t92 / f64x8::splat(2.0)));
            let t97 = t49 * t71;
            let t99 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t97)));
            let t102 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t99));
            let t103 = t102 * t27;
            let t107 = t56 * t83;
            let t110 = t6 * t107 * t65 / f64x8::splat(8.0);
            let t112 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t103 * t65 - t110));
            let tvrho0 = t47 + t69 + t7 * (t96 + t112);
            acc_vrho_0 = tvrho0;
            let t116 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t72)));
            let t119 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t116));
            let t120 = t119 * t27;
            let t125 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t120 * t43 - t87));
            let t127 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t97)));
            let t130 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t127));
            let t131 = t130 * t27;
            let t135 = t6 * t56;
            let t136 = t27 * t65;
            let t137 = f64x8::splat(1.0) / v_rho1;
            let t138 = param_ltafrac * t137;
            let t139 = t136 * t138;
            let t143 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t131 * t65 - t110 + t135 * t139 / f64x8::splat(2.0)));
            let tvrho1 = t47 + t69 + t7 * (t125 + t143);
            acc_vrho_1 = tvrho1;
            let tvsigma0 = f64x8::splat(0.0);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = f64x8::splat(0.0);
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t146 = f64x8::splat(1.0) / v_tau0;
            let t147 = param_ltafrac * t146;
            let t148 = t89 * t147;
            let t151 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(10.0) * t88 * t148));
            let tvtau0 = t7 * t151;
            acc_vtau_0 = tvtau0;
            let t152 = f64x8::splat(1.0) / v_tau1;
            let t153 = param_ltafrac * t152;
            let t154 = t136 * t153;
            let t157 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(10.0) * t135 * t154));
            let tvtau1 = t7 * t157;
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
