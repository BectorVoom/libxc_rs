//! MGGA_X_TH vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_th.c`
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
pub fn mgga_x_th_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
            let t3 = f64x8::splat(M_CBRTPI);
            let t4 = t3 * t3;
            let t5 = v_rho0 + v_rho1;
            let t6 = f64x8::splat(1.0) / t5;
            let t9 = (f64x8::splat(2.0) * v_rho0 * t6).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t13 = (f64x8::splat(2.0) * v_rho1 * t6).simd_le(zeta_threshold);
            let t14 = -t10;
            let t15 = v_rho0 - v_rho1;
            let t17 = ((t9).select(t10, (t13).select(t14, t15 * t6)));
            let t18 = f64x8::splat(1.0) + t17;
            let t19 = (t18).simd_le(zeta_threshold);
            let t20 = (simd::cbrt(zeta_threshold));
            let t21 = t20 * zeta_threshold;
            let t22 = (simd::cbrt(t18));
            let t24 = ((t19).select(t21, t22 * t18));
            let t25 = t4 * t24;
            let t26 = (simd::cbrt(t5));
            let t27 = f64x8::splat(1.0) / v_tau0;
            let t28 = t26 * t27;
            let t29 = t25 * t28;
            let t30 = (simd::cbrt(v_rho0));
            let t31 = t30 * t30;
            let t37 = f64x8::splat(1.0) + f64x8::splat(7.0) / f64x8::splat(216.0) * v_sigma0 / v_rho0 * t27;
            let t40 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t42 = f64x8::splat(M_CBRT4);
            let t43 = f64x8::splat(1.0) / t40 * t42;
            let t44 = t31 * v_rho0 * t37 * t43;
            let t47 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(80.0) * t29 * t44));
            let t48 = (v_rho1).simd_le(dens_threshold);
            let t49 = -t15;
            let t51 = ((t13).select(t10, (t9).select(t14, t49 * t6)));
            let t52 = f64x8::splat(1.0) + t51;
            let t53 = (t52).simd_le(zeta_threshold);
            let t54 = (simd::cbrt(t52));
            let t56 = ((t53).select(t21, t54 * t52));
            let t57 = t4 * t56;
            let t58 = f64x8::splat(1.0) / v_tau1;
            let t59 = t26 * t58;
            let t60 = t57 * t59;
            let t61 = (simd::cbrt(v_rho1));
            let t62 = t61 * t61;
            let t68 = f64x8::splat(1.0) + f64x8::splat(7.0) / f64x8::splat(216.0) * v_sigma2 / v_rho1 * t58;
            let t70 = t62 * v_rho1 * t68 * t43;
            let t73 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(80.0) * t60 * t70));
            let tzk0 = t47 + t73;
            acc_zk = tzk0;
            let t74 = t5 * t5;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t15 * t75;
            let t78 = ((t9).select(f64x8::splat(0.0), (t13).select(f64x8::splat(0.0), t6 - t76)));
            let t81 = ((t19).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t22 * t78));
            let t82 = t4 * t81;
            let t83 = t82 * t28;
            let t86 = t26 * t26;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t87 * t27;
            let t89 = t25 * t88;
            let t91 = f64x8::splat(9.0) / f64x8::splat(80.0) * t89 * t44;
            let t93 = t31 * t37 * t43;
            let t96 = v_tau0 * v_tau0;
            let t97 = f64x8::splat(1.0) / t96;
            let t98 = t26 * t97;
            let t99 = t25 * t98;
            let t100 = f64x8::splat(1.0) / t30;
            let t102 = t100 * v_sigma0 * t43;
            let t106 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(80.0) * t83 * t44 - t91 - f64x8::splat(9.0) / f64x8::splat(16.0) * t29 * t93 + f64x8::splat(7.0) / f64x8::splat(640.0) * t99 * t102));
            let t107 = t49 * t75;
            let t109 = ((t13).select(f64x8::splat(0.0), (t9).select(f64x8::splat(0.0), -t6 - t107)));
            let t112 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t109));
            let t113 = t4 * t112;
            let t114 = t113 * t59;
            let t117 = t87 * t58;
            let t118 = t57 * t117;
            let t120 = f64x8::splat(9.0) / f64x8::splat(80.0) * t118 * t70;
            let t122 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(80.0) * t114 * t70 - t120));
            let tvrho0 = t47 + t73 + t5 * (t106 + t122);
            acc_vrho_0 = tvrho0;
            let t126 = ((t9).select(f64x8::splat(0.0), (t13).select(f64x8::splat(0.0), -t6 - t76)));
            let t129 = ((t19).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t22 * t126));
            let t130 = t4 * t129;
            let t131 = t130 * t28;
            let t135 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(80.0) * t131 * t44 - t91));
            let t137 = ((t13).select(f64x8::splat(0.0), (t9).select(f64x8::splat(0.0), t6 - t107)));
            let t140 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t137));
            let t141 = t4 * t140;
            let t142 = t141 * t59;
            let t146 = t62 * t68 * t43;
            let t149 = v_tau1 * v_tau1;
            let t150 = f64x8::splat(1.0) / t149;
            let t151 = t26 * t150;
            let t152 = t57 * t151;
            let t153 = f64x8::splat(1.0) / t61;
            let t155 = t153 * v_sigma2 * t43;
            let t159 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(80.0) * t142 * t70 - t120 - f64x8::splat(9.0) / f64x8::splat(16.0) * t60 * t146 + f64x8::splat(7.0) / f64x8::splat(640.0) * t152 * t155));
            let tvrho1 = t47 + t73 + t5 * (t135 + t159);
            acc_vrho_1 = tvrho1;
            let t162 = t25 * t26;
            let t164 = t97 * t31 * t43;
            let t167 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(7.0) / f64x8::splat(640.0) * t162 * t164));
            let tvsigma0 = t5 * t167;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t168 = t57 * t26;
            let t170 = t150 * t62 * t43;
            let t173 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(7.0) / f64x8::splat(640.0) * t168 * t170));
            let tvsigma2 = t5 * t173;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t177 = f64x8::splat(1.0) / t96 / v_tau0;
            let t178 = t26 * t177;
            let t179 = t25 * t178;
            let t181 = t31 * v_sigma0 * t43;
            let t185 = ((t2).select(f64x8::splat(0.0), f64x8::splat(27.0) / f64x8::splat(80.0) * t99 * t44 + f64x8::splat(7.0) / f64x8::splat(640.0) * t179 * t181));
            let tvtau0 = t5 * t185;
            acc_vtau_0 = tvtau0;
            let t189 = f64x8::splat(1.0) / t149 / v_tau1;
            let t190 = t26 * t189;
            let t191 = t57 * t190;
            let t193 = t62 * v_sigma2 * t43;
            let t197 = ((t48).select(f64x8::splat(0.0), f64x8::splat(27.0) / f64x8::splat(80.0) * t152 * t70 + f64x8::splat(7.0) / f64x8::splat(640.0) * t191 * t193));
            let tvtau1 = t5 * t197;
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
