//! GGA_XC_TH2 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_5_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th2_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rmath::pow(rho0, 1.0 / 12.0);
        let t4 = rmath::pow(rho1, 1.0 / 12.0);
        let t7 = rmath::pow(rho0, 1.0 / 6.0);
        let t8 = t7 * rho0;
        let t10 = rmath::pow(rho1, 1.0 / 6.0);
        let t11 = t10 * rho1;
        let t13 = pow_1_3(rho0);
        let t14 = t13 * rho0;
        let t16 = pow_1_3(rho1);
        let t17 = t16 * rho1;
        let t19 = rmath::sqrt(rho0);
        let t20 = t19 * rho0;
        let t22 = rmath::sqrt(rho1);
        let t23 = t22 * rho1;
        let t25 = t13 * t13;
        let t26 = t25 * rho0;
        let t28 = t16 * t16;
        let t29 = t28 * rho1;
        let t31 = t1 * t1;
        let t32 = t31 * t31;
        let t33 = t32 * t1;
        let t35 = t4 * t4;
        let t36 = t35 * t35;
        let t37 = t36 * t4;
        let t39 = rho0 * t33 + rho1 * t37;
        let t40 = rmath::sqrt(sigma0);
        let t41 = 1.0 / t14;
        let t42 = t40 * t41;
        let t43 = rho0 - rho1;
        let t44 = rho0 + rho1;
        let t45 = 1.0 / t44;
        let t46 = t43 * t45;
        let t47 = 1.0 + t46;
        let t48 = t47 <= zeta_threshold;
        let t49 = pow_1_3(zeta_threshold);
        let t50 = t49 * zeta_threshold;
        let t51 = pow_1_3(t47);
        let t53 = piecewise3(t48, t50, t51 * t47);
        let t54 = M_CBRT2;
        let t55 = t54 * t54;
        let t56 = t53 * t55;
        let t58 = rmath::sqrt(sigma2);
        let t59 = 1.0 / t17;
        let t60 = t58 * t59;
        let t61 = 1.0 - t46;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t65 = piecewise3(t62, t50, t63 * t61);
        let t66 = t65 * t55;
        let t69 = t42 * t56 / 4.0 + t60 * t66 / 4.0;
        let t72 = t20 + t23;
        let t75 = 0.678831 * t1 * rho0 + 0.678831 * t4 * rho1 - 1.75821 * t8 - 1.75821 * t11 + 1.27676 * t14 + 1.27676 * t17 - 1.60789 * t20 - 1.60789 * t23 + 0.36561 * t26 + 0.36561 * t29 - 0.0906635 * t39 * t69 + 0.0734865 * t72 * t69;
        let t76 = t26 + t29;
        let t79 = t7 * t7;
        let t80 = t79 * t79;
        let t81 = t80 * t7;
        let t82 = t81 * rho0;
        let t83 = t10 * t10;
        let t84 = t83 * t83;
        let t85 = t84 * t10;
        let t86 = t85 * rho1;
        let t87 = t82 + t86;
        let t90 = rho0 * rho0;
        let t92 = 1.0 / t25 / t90;
        let t93 = sigma0 * t92;
        let t94 = t53 * t53;
        let t95 = t94 * t54;
        let t96 = t93 * t95;
        let t97 = rho1 * rho1;
        let t99 = 1.0 / t28 / t97;
        let t100 = sigma2 * t99;
        let t101 = t65 * t65;
        let t102 = t101 * t54;
        let t103 = t100 * t102;
        let t105 = t96 / 8.0 + t103 / 8.0;
        let t110 = t90 + t97;
        let t116 = sigma0 + 2.0 * sigma1 + sigma2;
        let t117 = t44 * t44;
        let t118 = pow_1_3(t44);
        let t119 = t118 * t118;
        let t121 = 1.0 / t119 / t117;
        let t122 = t116 * t121;
        let t123 = t96 / 4.0 + t103 / 4.0 - t122;
        let t130 = t8 + t11;
        let t131 = t43 * t43;
        let t132 = t130 * t131;
        let t133 = 1.0 / t117;
        let t136 = t14 + t17;
        let t137 = t136 * t131;
        let t140 = t72 * t131;
        let t143 = pow_5_3(rho0);
        let t144 = pow_5_3(rho1);
        let t145 = t143 + t144;
        let t146 = t145 * t131;
        let t149 = 0.0735705 * t76 * t69 - 0.03584585 * t87 * t69 - 0.02035835 * t76 * t105 + 0.01073125 * t87 * t105 - 0.000384078 * t110 * t105 + 0.0310377 * t76 * t123 - 0.0720326 * t87 * t123 + 0.0446562 * t110 * t123 - 0.266802 * t132 * t133 + 1.50822 * t137 * t133 - 1.94515 * t140 * t133 + 0.679078 * t146 * t133;
        let tzk0 = (t75 + t149) * t45;
        zk[ip] += tzk0;
    }
}
