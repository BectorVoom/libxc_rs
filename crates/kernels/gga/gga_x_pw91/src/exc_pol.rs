//! GGA_X_PW91 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw91.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pw91_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t18 = piecewise5::<f64>(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_alpha * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = t34 * t39;
        let t43 = f64::exp(-t29 * t40 / 24.0);
        let t46 = (param_d * t43 + param_c) * t28;
        let t49 = t28 * t28;
        let t50 = 1.0 / t31;
        let t51 = t49 * t50;
        let t52 = f64::sqrt(sigma0);
        let t54 = 1.0 / t36 / rho0;
        let t58 = f64::powf(t51 * t52 * t54 / 12.0, param_expo);
        let t59 = param_f * t58;
        let t60 = t46 * t40 / 24.0 - t59;
        let t61 = t51 * t52;
        let t63 = param_b * t49;
        let t68 = f64::ln(t63 * t50 * t52 * t54 / 12.0 + f64::sqrt(pow_2::<f64>(t63 * t50 * t52 * t54 / 12.0) + 1.0));
        let t69 = t54 * param_a * t68;
        let t72 = 1.0 + t61 * t69 / 12.0 + t59;
        let t73 = 1.0 / t72;
        let t75 = t60 * t73 + 1.0;
        let t79 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t75);
        let t80 = rho1 <= dens_threshold;
        let t81 = -t16;
        let t83 = piecewise5::<f64>(t14, t11, t10, t15, t81 * t7);
        let t84 = 1.0 + t83;
        let t85 = t84 <= zeta_threshold;
        let t86 = pow_1_3::<f64>(t84);
        let t88 = piecewise3::<f64>(t85, t22, t86 * t84);
        let t89 = t88 * t26;
        let t90 = t33 * sigma2;
        let t91 = rho1 * rho1;
        let t92 = pow_1_3::<f64>(rho1);
        let t93 = t92 * t92;
        let t95 = 1.0 / t93 / t91;
        let t96 = t90 * t95;
        let t99 = f64::exp(-t29 * t96 / 24.0);
        let t102 = (param_d * t99 + param_c) * t28;
        let t105 = f64::sqrt(sigma2);
        let t107 = 1.0 / t92 / rho1;
        let t111 = f64::powf(t51 * t105 * t107 / 12.0, param_expo);
        let t112 = param_f * t111;
        let t113 = t102 * t96 / 24.0 - t112;
        let t114 = t51 * t105;
        let t120 = f64::ln(t63 * t50 * t105 * t107 / 12.0 + f64::sqrt(pow_2::<f64>(t63 * t50 * t105 * t107 / 12.0) + 1.0));
        let t121 = t107 * param_a * t120;
        let t124 = 1.0 + t114 * t121 / 12.0 + t112;
        let t125 = 1.0 / t124;
        let t127 = t113 * t125 + 1.0;
        let t131 = piecewise3::<f64>(t80, 0.0, -3.0 / 8.0 * t5 * t89 * t127);
        let tzk0 = t79 + t131;
        zk[ip] += tzk0;
    }
}
