//! GGA_X_AM05 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_am05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::lambert_w::{lambert_w};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_am05_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_alpha: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = param_alpha * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = t26 * t34;
        let t37 = 1.0 + t35 / 24.0;
        let t38 = 1.0 / t37;
        let t39 = t33 * t38;
        let t43 = t25 * sigma[ip];
        let t44 = t21 * t43;
        let t45 = t28 * t33;
        let t46 = param_c * t20;
        let t47 = t46 * t25;
        let t50 = 1.0 + t47 * t34 / 24.0;
        let t51 = t38 * t50;
        let t52 = t20 * t20;
        let t53 = param_c * t52;
        let t54 = 1.0 / t23;
        let t55 = rmath::sqrt(sigma[ip]);
        let t56 = t54 * t55;
        let t58 = t53 * t56 * t28;
        let t60 = 1.0 / t18 / rho[ip];
        let t61 = 1.0 / M_PI;
        let t62 = t60 * t61;
        let t63 = t3 * t3;
        let t64 = rmath::sqrt(12.0);
        let t68 = t52 * t54 * t55 * t27 * t60;
        let t69 = rmath::sqrt(t68);
        let t72 = rmath::sqrt(6.0);
        let t75 = lambert_w(t64 * t69 * t68 * t72 / 1728.0);
        let t76 = pow_1_3(t75);
        let t77 = t76 * t76;
        let t83 = 28.23705740248932 + 3.0 / 4.0 * t3 * t28 * t76 * t75;
        let t84 = pow_1_4(t83);
        let t85 = t63 * t77 * t84;
        let t86 = t62 * t85;
        let t89 = 1.0 + t58 * t86 / 8.0;
        let t90 = 1.0 / t89;
        let t91 = t51 * t90;
        let t95 = 1.0 - t26 * t29 * t39 / 24.0 + t44 * t45 * t91 / 24.0;
        let t99 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t95);
        let tzk0 = 2.0 * t99;
        zk[ip] += tzk0;
    }
}
