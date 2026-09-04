//! GGA_XC_TH3 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th3.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th3_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_18: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_omega_0 = f64x8::splat(param_omega_0);
    let param_omega_1 = f64x8::splat(param_omega_1);
    let param_omega_2 = f64x8::splat(param_omega_2);
    let param_omega_3 = f64x8::splat(param_omega_3);
    let param_omega_4 = f64x8::splat(param_omega_4);
    let param_omega_5 = f64x8::splat(param_omega_5);
    let param_omega_6 = f64x8::splat(param_omega_6);
    let param_omega_7 = f64x8::splat(param_omega_7);
    let param_omega_8 = f64x8::splat(param_omega_8);
    let param_omega_9 = f64x8::splat(param_omega_9);
    let param_omega_10 = f64x8::splat(param_omega_10);
    let param_omega_11 = f64x8::splat(param_omega_11);
    let param_omega_12 = f64x8::splat(param_omega_12);
    let param_omega_13 = f64x8::splat(param_omega_13);
    let param_omega_18 = f64x8::splat(param_omega_18);
    let param_omega_14 = f64x8::splat(param_omega_14);
    let param_omega_15 = f64x8::splat(param_omega_15);
    let param_omega_16 = f64x8::splat(param_omega_16);
    let param_omega_17 = f64x8::splat(param_omega_17);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (simd::pow(f64x8::splat(2.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t3 = t2 * t2;
            let t4 = t3 * t3;
            let t6 = param_omega_0 * t4 * t2;
            let t7 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t8 = t7 * v_rho;
            let t12 = f64x8::splat(M_CBRT2);
            let t13 = t12 * t12;
            let t14 = param_omega_1 * t13;
            let t15 = (simd::cbrt(v_rho));
            let t16 = t15 * v_rho;
            let t20 = f64x8::splat(M_SQRT2);
            let t21 = param_omega_2 * t20;
            let t22 = ((v_rho).sqrt());
            let t23 = t22 * v_rho;
            let t27 = param_omega_3 * t12;
            let t28 = t15 * t15;
            let t29 = t28 * v_rho;
            let t33 = (simd::pow(f64x8::splat(2.0), f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t34 = t33 * t33;
            let t36 = t34 * t34;
            let t38 = param_omega_4 * t36 * t34 * t33;
            let t39 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t40 = ((v_sigma).sqrt());
            let t43 = (simd::cbrt(zeta_threshold));
            let t45 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t43 * zeta_threshold, f64x8::splat(1.0)));
            let t50 = param_omega_5 * t20;
            let t56 = param_omega_6 * t12;
            let t62 = param_omega_7 * t2;
            let t68 = param_omega_8 * t12;
            let t69 = f64x8::splat(1.0) / v_rho;
            let t71 = t45 * t45;
            let t76 = param_omega_9 * t2;
            let t77 = t7 * t7;
            let t78 = t77 * t77;
            let t79 = t78 * t7;
            let t80 = f64x8::splat(1.0) / t79;
            let t85 = param_omega_10;
            let t86 = f64x8::splat(1.0) / t28;
            let t87 = t85 * t86;
            let t88 = v_sigma * t71;
            let t92 = param_omega_11 * t12;
            let t93 = v_rho * v_rho;
            let t95 = f64x8::splat(1.0) / t28 / t93;
            let t96 = v_sigma * t95;
            let t98 = t96 * t71 - t96;
            let t103 = param_omega_12 * t2;
            let t104 = t79 * v_rho;
            let t108 = param_omega_13;
            let t109 = t108 * t93;
            let t112 = param_omega_18;
            let t113 = (simd::pow(v_rho, f64x8::splat(1.0833333333333333)));
            let t116 = t6 * t8 / f64x8::splat(2.0) + t14 * t16 / f64x8::splat(2.0) + t21 * t23 / f64x8::splat(2.0) + t27 * t29 / f64x8::splat(2.0) + t38 * t39 * t40 * t45 / f64x8::splat(4.0) + t50 * t7 * t40 * t45 / f64x8::splat(4.0) + t56 * t15 * t40 * t45 / f64x8::splat(4.0) + t62 * t22 * t40 * t45 / f64x8::splat(4.0) + t68 * t69 * v_sigma * t71 / f64x8::splat(8.0) + t76 * t80 * v_sigma * t71 / f64x8::splat(8.0) + t87 * t88 / f64x8::splat(8.0) + t92 * t29 * t98 / f64x8::splat(2.0) + t103 * t104 * t98 / f64x8::splat(2.0) + t109 * t98 / f64x8::splat(2.0) + f64x8::splat(0.9438743126816935) * t112 * t113;
            let tzk0 = t116 * t69;
            acc_zk = tzk0;
            let t125 = t39 * t39;
            let t127 = t125 * t125;
            let t128 = t127 * t127;
            let t129 = t128 * t125 * t39;
            let t130 = f64x8::splat(1.0) / t129;
            let t143 = f64x8::splat(1.0) / t22;
            let t148 = f64x8::splat(1.0) / t93;
            let t153 = f64x8::splat(1.0) / t104;
            let t158 = f64x8::splat(1.0) / t29;
            let t159 = t85 * t158;
            let t165 = t93 * v_rho;
            let t167 = f64x8::splat(1.0) / t28 / t165;
            let t168 = v_sigma * t167;
            let t171 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t168 * t71 + f64x8::splat(8.0) / f64x8::splat(3.0) * t168;
            let t181 = t108 * v_rho;
            let t185 = (simd::pow(v_rho, f64x8::splat(0.08333333333333333)));
            let tvrho0 = f64x8::splat(7.0) / f64x8::splat(12.0) * t6 * t7 + f64x8::splat(2.0) / f64x8::splat(3.0) * t14 * t15 + f64x8::splat(3.0) / f64x8::splat(4.0) * t21 * t22 + f64x8::splat(5.0) / f64x8::splat(6.0) * t27 * t28 + t38 * t130 * t40 * t45 / f64x8::splat(48.0) + t50 * t80 * t40 * t45 / f64x8::splat(24.0) + t56 * t86 * t40 * t45 / f64x8::splat(12.0) + t62 * t143 * t40 * t45 / f64x8::splat(8.0) - t68 * t148 * v_sigma * t71 / f64x8::splat(8.0) - f64x8::splat(5.0) / f64x8::splat(48.0) * t76 * t153 * v_sigma * t71 - t159 * t88 / f64x8::splat(12.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t92 * t28 * t98 + t92 * t29 * t171 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t103 * t79 * t98 + t103 * t104 * t171 / f64x8::splat(2.0) + t181 * t98 + t109 * t171 / f64x8::splat(2.0) + f64x8::splat(1.0225305054051679) * t112 * t185;
            acc_vrho = tvrho0;
            let t188 = f64x8::splat(1.0) / t40;
            let t214 = t95 * t71 - t95;
            let tvsigma0 = t38 * t39 * t188 * t45 / f64x8::splat(8.0) + t50 * t7 * t188 * t45 / f64x8::splat(8.0) + t56 * t15 * t188 * t45 / f64x8::splat(8.0) + t62 * t22 * t188 * t45 / f64x8::splat(8.0) + t68 * t69 * t71 / f64x8::splat(8.0) + t76 * t80 * t71 / f64x8::splat(8.0) + t87 * t71 / f64x8::splat(8.0) + t92 * t29 * t214 / f64x8::splat(2.0) + t103 * t104 * t214 / f64x8::splat(2.0) + t109 * t214 / f64x8::splat(2.0);
            acc_vsigma = tvsigma0;
            let t229 = f64x8::splat(1.0) / t15;
            let t233 = f64x8::splat(1.0) / t129 / v_rho;
            let t246 = f64x8::splat(1.0) / t23;
            let t251 = f64x8::splat(1.0) / t165;
            let t257 = f64x8::splat(1.0) / t79 / t93;
            let t263 = t85 * t95;
            let t272 = t93 * t93;
            let t274 = f64x8::splat(1.0) / t28 / t272;
            let t275 = v_sigma * t274;
            let t278 = f64x8::splat(88.0) / f64x8::splat(9.0) * t275 * t71 - f64x8::splat(88.0) / f64x8::splat(9.0) * t275;
            let t282 = f64x8::splat(1.0) / t7;
            let t297 = (simd::pow(v_rho, -f64x8::splat(0.9166666666666666)));
            let t300 = f64x8::splat(5.0) / f64x8::splat(36.0) * t263 * t88 + f64x8::splat(5.0) / f64x8::splat(9.0) * t92 * t229 * t98 + f64x8::splat(5.0) / f64x8::splat(3.0) * t92 * t28 * t171 + t92 * t29 * t278 / f64x8::splat(2.0) + f64x8::splat(55.0) / f64x8::splat(72.0) * t103 * t282 * t98 + f64x8::splat(11.0) / f64x8::splat(6.0) * t103 * t79 * t171 + t103 * t104 * t278 / f64x8::splat(2.0) + t108 * t98 + f64x8::splat(2.0) * t181 * t171 + t109 * t278 / f64x8::splat(2.0) + f64x8::splat(0.08521087545043066) * t112 * t297;
            let tv2rho20 = f64x8::splat(7.0) / f64x8::splat(72.0) * t6 * t80 + f64x8::splat(2.0) / f64x8::splat(9.0) * t14 * t86 + f64x8::splat(3.0) / f64x8::splat(8.0) * t21 * t143 + f64x8::splat(5.0) / f64x8::splat(9.0) * t27 * t229 - f64x8::splat(11.0) / f64x8::splat(576.0) * t38 * t233 * t40 * t45 - f64x8::splat(5.0) / f64x8::splat(144.0) * t50 * t153 * t40 * t45 - t56 * t158 * t40 * t45 / f64x8::splat(18.0) - t62 * t246 * t40 * t45 / f64x8::splat(16.0) + t68 * t251 * v_sigma * t71 / f64x8::splat(4.0) + f64x8::splat(55.0) / f64x8::splat(288.0) * t76 * t257 * v_sigma * t71 + t300;
            acc_v2rho2 = tv2rho20;
            let t330 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t167 * t71 + f64x8::splat(8.0) / f64x8::splat(3.0) * t167;
            let tv2rhosigma0 = t38 * t130 * t188 * t45 / f64x8::splat(96.0) + t50 * t80 * t188 * t45 / f64x8::splat(48.0) + t56 * t86 * t188 * t45 / f64x8::splat(24.0) + t62 * t143 * t188 * t45 / f64x8::splat(16.0) - t68 * t148 * t71 / f64x8::splat(8.0) - f64x8::splat(5.0) / f64x8::splat(48.0) * t76 * t153 * t71 - t159 * t71 / f64x8::splat(12.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t92 * t28 * t214 + t92 * t29 * t330 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t103 * t79 * t214 + t103 * t104 * t330 / f64x8::splat(2.0) + t181 * t214 + t109 * t330 / f64x8::splat(2.0);
            acc_v2rhosigma = tv2rhosigma0;
            let t344 = f64x8::splat(1.0) / t40 / v_sigma;
            let tv2sigma20 = -t56 * t15 * t344 * t45 / f64x8::splat(16.0) - t62 * t22 * t344 * t45 / f64x8::splat(16.0) - t38 * t39 * t344 * t45 / f64x8::splat(16.0) - t50 * t7 * t344 * t45 / f64x8::splat(16.0);
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
