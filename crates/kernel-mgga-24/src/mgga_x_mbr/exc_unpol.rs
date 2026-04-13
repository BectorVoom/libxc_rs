//! MGGA_X_MBR exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbr.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mbr_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
    param_lambda: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = pow_1_3(rho[ip]);
        let t16 = t14 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = t16 * t19;
        let t21 = M_CBRT4;
        let t22 = param_lambda * param_lambda;
        let t23 = t22 - param_lambda + 1.0 / 2.0;
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = tau[ip] * t25;
        let t27 = t15 * t15;
        let t29 = 1.0 / t27 / rho[ip];
        let t31 = 2.0 * t26 * t29;
        let t32 = M_CBRT6;
        let t33 = t32 * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = t33 * t36;
        let t39 = sigma[ip] * t25;
        let t40 = rho[ip] * rho[ip];
        let t42 = 1.0 / t27 / t40;
        let t43 = t39 * t42;
        let t49 = pow_2(2.0 * param_lambda - 1.0);
        let t50 = t49 * t32;
        let t51 = 1.0 / t36;
        let t52 = t50 * t51;
        let t55 = t49 * t49;
        let t56 = param_beta * t55;
        let t57 = t56 * t33;
        let t59 = 1.0 / t35 / t34;
        let t60 = sigma[ip] * sigma[ip];
        let t61 = t59 * t60;
        let t62 = t40 * t40;
        let t63 = t62 * rho[ip];
        let t65 = 1.0 / t15 / t63;
        let t66 = t24 * t65;
        let t70 = 1.0 + 175.0 / 162.0 * t52 * t43 + t57 * t61 * t66 / 288.0;
        let t71 = f64::powf(t70, 1.0 / 5.0);
        let t75 = t49 * sigma[ip];
        let t76 = t25 * t42;
        let t82 = t23 * (t31 - 3.0 / 5.0 * t37 - t43 / 36.0) + t37 * (t71 - 1.0) / 5.0 - param_gamma * (t31 - t75 * t76 / 4.0) / 3.0;
        let t83 = f64::abs(t82);
        let t84 = t83 < 0.5e-12;
        let t85 = 0.0 < t82;
        let t86 = piecewise3(t85, 0.5e-12, -0.5e-12);
        let t87 = piecewise3(t84, t86, t82);
        let t88 = xc_mgga_x_br89_get_x(t87);
        let t90 = f64::exp(t88 / 3.0);
        let t91 = t21 * t90;
        let t92 = f64::exp(-t88);
        let t94 = 1.0 + t88 / 2.0;
        let t95 = t92 * t94;
        let t96 = 1.0 - t95;
        let t97 = 1.0 / t88;
        let t98 = t96 * t97;
        let t99 = t91 * t98;
        let t102 = piecewise3(t3, 0.0, -t20 * t99 / 4.0);
        let tzk0 = 2.0 * t102;
        zk[ip] += tzk0;
    }
}
