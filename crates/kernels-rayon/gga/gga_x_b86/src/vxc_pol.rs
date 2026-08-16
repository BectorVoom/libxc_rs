//! GGA_X_B86 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b86.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_b86_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    param_omega: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = param_beta * sigma0;
        let t29 = rho0 * rho0;
        let t30 = pow_1_3(rho0);
        let t31 = t30 * t30;
        let t33 = 1.0 / t31 / t29;
        let t36 = param_gamma * sigma0 * t33 + 1.0;
        let t37 = f64::powf(t36, param_omega);
        let t38 = 1.0 / t37;
        let t41 = t28 * t33 * t38 + 1.0;
        let t45 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t41);
        let t46 = rho1 <= dens_threshold;
        let t47 = -t16;
        let t49 = piecewise5(t14, t11, t10, t15, t47 * t7);
        let t50 = 1.0 + t49;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t54 = piecewise3(t51, t22, t52 * t50);
        let t55 = t54 * t26;
        let t56 = param_beta * sigma2;
        let t57 = rho1 * rho1;
        let t58 = pow_1_3(rho1);
        let t59 = t58 * t58;
        let t61 = 1.0 / t59 / t57;
        let t64 = param_gamma * sigma2 * t61 + 1.0;
        let t65 = f64::powf(t64, param_omega);
        let t66 = 1.0 / t65;
        let t69 = t56 * t61 * t66 + 1.0;
        let t73 = piecewise3(t46, 0.0, -3.0 / 8.0 * t5 * t55 * t69);
        let tzk0 = t45 + t73;
        zk[ip] += tzk0;
        let t74 = t6 * t6;
        let t75 = 1.0 / t74;
        let t76 = t16 * t75;
        let t78 = piecewise5(t10, 0.0, t14, 0.0, t7 - t76);
        let t81 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t78);
        let t82 = t81 * t26;
        let t86 = t26 * t26;
        let t87 = 1.0 / t86;
        let t88 = t25 * t87;
        let t91 = t5 * t88 * t41 / 8.0;
        let t92 = t29 * rho0;
        let t94 = 1.0 / t31 / t92;
        let t97 = sigma0 * sigma0;
        let t98 = param_beta * t97;
        let t99 = t29 * t29;
        let t100 = t99 * t29;
        let t102 = 1.0 / t30 / t100;
        let t104 = t38 * param_omega;
        let t105 = 1.0 / t36;
        let t107 = t104 * param_gamma * t105;
        let t110 = 8.0 / 3.0 * t98 * t102 * t107 - 8.0 / 3.0 * t28 * t94 * t38;
        let t115 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t82 * t41 - t91 - 3.0 / 8.0 * t5 * t27 * t110);
        let t116 = t47 * t75;
        let t118 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t116);
        let t121 = piecewise3(t51, 0.0, 4.0 / 3.0 * t52 * t118);
        let t122 = t121 * t26;
        let t126 = t54 * t87;
        let t129 = t5 * t126 * t69 / 8.0;
        let t131 = piecewise3(t46, 0.0, -3.0 / 8.0 * t5 * t122 * t69 - t129);
        let tvrho0 = t45 + t73 + t6 * (t115 + t131);
        vrho[ip * 2] += tvrho0;
        let t135 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t76);
        let t138 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t135);
        let t139 = t138 * t26;
        let t144 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t139 * t41 - t91);
        let t146 = piecewise5(t14, 0.0, t10, 0.0, t7 - t116);
        let t149 = piecewise3(t51, 0.0, 4.0 / 3.0 * t52 * t146);
        let t150 = t149 * t26;
        let t154 = t57 * rho1;
        let t156 = 1.0 / t59 / t154;
        let t159 = sigma2 * sigma2;
        let t160 = param_beta * t159;
        let t161 = t57 * t57;
        let t162 = t161 * t57;
        let t164 = 1.0 / t58 / t162;
        let t166 = t66 * param_omega;
        let t167 = 1.0 / t64;
        let t169 = t166 * param_gamma * t167;
        let t172 = -8.0 / 3.0 * t56 * t156 * t66 + 8.0 / 3.0 * t160 * t164 * t169;
        let t177 = piecewise3(t46, 0.0, -3.0 / 8.0 * t5 * t150 * t69 - t129 - 3.0 / 8.0 * t5 * t55 * t172);
        let tvrho1 = t45 + t73 + t6 * (t144 + t177);
        vrho[ip * 2 + 1] += tvrho1;
        let t182 = t99 * rho0;
        let t184 = 1.0 / t30 / t182;
        let t187 = -t28 * t184 * t107 + param_beta * t33 * t38;
        let t191 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t187);
        let tvsigma0 = t6 * t191;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t194 = t161 * rho1;
        let t196 = 1.0 / t58 / t194;
        let t199 = -t56 * t196 * t169 + param_beta * t61 * t66;
        let t203 = piecewise3(t46, 0.0, -3.0 / 8.0 * t5 * t55 * t199);
        let tvsigma2 = t6 * t203;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
