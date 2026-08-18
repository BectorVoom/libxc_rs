//! MGGA_X_MVS vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvs.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mvs_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t7 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = tau[ip] * t22;
        let t24 = t20 * t20;
        let t26 = 1.0 / t24 / rho[ip];
        let t28 = sigma[ip] * t22;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t24 / t29;
        let t34 = t23 * t26 - t28 * t31 / 8.0;
        let t35 = M_CBRT6;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3(t37);
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t44 = param_k0 * (1.0 - 5.0 / 9.0 * t34 * t35 * t40);
        let t45 = t34 * t34;
        let t47 = t35 * t35;
        let t49 = 1.0 / t38 / t37;
        let t50 = t47 * t49;
        let t53 = 1.0 + 25.0 / 81.0 * param_e1 * t45 * t50;
        let t54 = t53 * t53;
        let t55 = t45 * t45;
        let t57 = t37 * t37;
        let t59 = 1.0 / t39 / t57;
        let t60 = t35 * t59;
        let t63 = t54 + 1250.0 / 2187.0 * param_c1 * t55 * t60;
        let t64 = pow_1_4(t63);
        let t65 = 1.0 / t64;
        let t67 = t44 * t65 + 1.0;
        let t71 = sigma[ip] * sigma[ip];
        let t73 = t29 * t29;
        let t74 = t73 * rho[ip];
        let t76 = 1.0 / t20 / t74;
        let t80 = 1.0 + param_b * t47 * t49 * t71 * t21 * t76 / 288.0;
        let t81 = f64::powf(t80, 1.0 / 8.0);
        let t82 = 1.0 / t81;
        let t86 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t67 * t82);
        let tzk0 = 2.0 * t86;
        zk[ip] += tzk0;
        let t87 = 1.0 / t24;
        let t94 = t29 * rho[ip];
        let t96 = 1.0 / t24 / t94;
        let t99 = -5.0 / 3.0 * t23 * t31 + t28 * t96 / 3.0;
        let t100 = param_k0 * t99;
        let t101 = t35 * t40;
        let t102 = t101 * t65;
        let t106 = 1.0 / t64 / t63;
        let t107 = t53 * param_e1;
        let t108 = t107 * t34;
        let t113 = param_c1 * t45 * t34;
        let t117 = 100.0 / 81.0 * t108 * t50 * t99 + 5000.0 / 2187.0 * t113 * t60 * t99;
        let t118 = t106 * t117;
        let t121 = -5.0 / 9.0 * t100 * t102 - t44 * t118 / 4.0;
        let t126 = t73 * t29;
        let t127 = 1.0 / t126;
        let t128 = t18 * t127;
        let t130 = t7 * t128 * t67;
        let t133 = 1.0 / t81 / t80 * param_b;
        let t134 = t133 * t47;
        let t137 = t134 * t49 * t71 * t21;
        let t141 = piecewise3(t3, 0.0, -t19 * t87 * t67 * t82 / 8.0 - 3.0 / 8.0 * t19 * t20 * t121 * t82 - t130 * t137 / 1152.0);
        let tvrho0 = 2.0 * rho[ip] * t141 + 2.0 * t86;
        vrho[ip] += tvrho0;
        let t144 = param_k0 * t22;
        let t145 = t144 * t31;
        let t146 = t145 * t102;
        let t148 = t22 * t31;
        let t149 = t50 * t148;
        let t150 = t108 * t149;
        let t152 = t113 * t35;
        let t153 = t59 * t22;
        let t154 = t153 * t31;
        let t155 = t152 * t154;
        let t157 = -25.0 / 162.0 * t150 - 625.0 / 2187.0 * t155;
        let t158 = t106 * t157;
        let t161 = 5.0 / 72.0 * t146 - t44 * t158 / 4.0;
        let t166 = 1.0 / t74;
        let t167 = t18 * t166;
        let t169 = t7 * t167 * t67;
        let t172 = t134 * t49 * sigma[ip] * t21;
        let t176 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t161 * t82 + t169 * t172 / 3072.0);
        let tvsigma0 = 2.0 * rho[ip] * t176;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t178 = t144 * t26;
        let t181 = t22 * t26;
        let t182 = t50 * t181;
        let t185 = t153 * t26;
        let t188 = 100.0 / 81.0 * t108 * t182 + 5000.0 / 2187.0 * t152 * t185;
        let t189 = t106 * t188;
        let t192 = -5.0 / 9.0 * t178 * t102 - t44 * t189 / 4.0;
        let t197 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t192 * t82);
        let tvtau0 = 2.0 * rho[ip] * t197;
        vtau[ip] += tvtau0;
    }
}
