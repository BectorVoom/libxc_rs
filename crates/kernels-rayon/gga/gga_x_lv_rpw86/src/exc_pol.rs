//! GGA_X_LV_RPW86 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lv_rpw86.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lv_rpw86_exc_pol(
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
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t33 = t28 / t31;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t40 = t33 * sigma0 * t38;
        let t42 = 1.0 + 0.003931018518518519 * t40;
        let t43 = sigma0 * sigma0;
        let t44 = t43 * sigma0;
        let t45 = t34 * t34;
        let t46 = t45 * t45;
        let t47 = 1.0 / t46;
        let t48 = t44 * t47;
        let t49 = 9.704561350131286e-08 * t48;
        let t50 = 1.0 + t49;
        let t51 = 1.0 / t50;
        let t54 = t28 * t28;
        let t57 = t54 / t30 / t29;
        let t58 = t45 * rho0;
        let t60 = 1.0 / t35 / t58;
        let t65 = 1.0 + 0.077125 * t40 + 0.030086805555555554 * t57 * t43 * t60 + 7.26282598747199e-07 * t48;
        let t66 = rmath::pow(t65, 1.0 / 15.0);
        let t67 = 1.15 + t49;
        let t68 = 1.0 / t67;
        let t69 = t66 * t68;
        let t72 = t42 * t51 + 9.704561350131286e-08 * t48 * t69;
        let t76 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t72);
        let t77 = rho1 <= dens_threshold;
        let t78 = -t16;
        let t80 = piecewise5(t14, t11, t10, t15, t78 * t7);
        let t81 = 1.0 + t80;
        let t82 = t81 <= zeta_threshold;
        let t83 = pow_1_3(t81);
        let t85 = piecewise3(t82, t22, t83 * t81);
        let t86 = t85 * t26;
        let t87 = rho1 * rho1;
        let t88 = pow_1_3(rho1);
        let t89 = t88 * t88;
        let t91 = 1.0 / t89 / t87;
        let t93 = t33 * sigma2 * t91;
        let t95 = 1.0 + 0.003931018518518519 * t93;
        let t96 = sigma2 * sigma2;
        let t97 = t96 * sigma2;
        let t98 = t87 * t87;
        let t99 = t98 * t98;
        let t100 = 1.0 / t99;
        let t101 = t97 * t100;
        let t102 = 9.704561350131286e-08 * t101;
        let t103 = 1.0 + t102;
        let t104 = 1.0 / t103;
        let t107 = t98 * rho1;
        let t109 = 1.0 / t88 / t107;
        let t114 = 1.0 + 0.077125 * t93 + 0.030086805555555554 * t57 * t96 * t109 + 7.26282598747199e-07 * t101;
        let t115 = rmath::pow(t114, 1.0 / 15.0);
        let t116 = 1.15 + t102;
        let t117 = 1.0 / t116;
        let t118 = t115 * t117;
        let t121 = t95 * t104 + 9.704561350131286e-08 * t101 * t118;
        let t125 = piecewise3(t77, 0.0, -3.0 / 8.0 * t5 * t86 * t121);
        let tzk0 = t76 + t125;
        zk[ip] += tzk0;
    }
}
