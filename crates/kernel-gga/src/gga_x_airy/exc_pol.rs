//! GGA_X_AIRY exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_airy_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t29 = t28 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t34 = f64::sqrt(sigma0);
        let t35 = pow_1_3(rho0);
        let t37 = 1.0 / t35 / rho0;
        let t39 = t33 * t34 * t37;
        let t40 = f64::powf(t39, 0.2626712e1);
        let t42 = 1.0 + 0.13471619689594796103e-3 * t40;
        let t43 = f64::powf(t42, -0.657946e0);
        let t46 = f64::powf(t39, 0.3217063e1);
        let t48 = f64::powf(t39, 0.3223476e1);
        let t50 = 1.0 - 0.45212413010769857073e-1 * t46 + 0.45402221956620378581e-1 * t48;
        let t51 = f64::powf(t39, 0.3473804e1);
        let t53 = 1.0 + 0.47702180224903349918e-3 * t51;
        let t54 = 1.0 / t53;
        let t56 = 0.60146019220211109872e-4 * t40 * t43 + t50 * t54;
        let t60 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t56);
        let t61 = rho1 <= dens_threshold;
        let t62 = -t16;
        let t64 = piecewise5(t14, t11, t10, t15, t62 * t7);
        let t65 = 1.0 + t64;
        let t66 = t65 <= zeta_threshold;
        let t67 = pow_1_3(t65);
        let t69 = piecewise3(t66, t22, t67 * t65);
        let t70 = t69 * t26;
        let t71 = f64::sqrt(sigma2);
        let t72 = pow_1_3(rho1);
        let t74 = 1.0 / t72 / rho1;
        let t76 = t33 * t71 * t74;
        let t77 = f64::powf(t76, 0.2626712e1);
        let t79 = 1.0 + 0.13471619689594796103e-3 * t77;
        let t80 = f64::powf(t79, -0.657946e0);
        let t83 = f64::powf(t76, 0.3217063e1);
        let t85 = f64::powf(t76, 0.3223476e1);
        let t87 = 1.0 - 0.45212413010769857073e-1 * t83 + 0.45402221956620378581e-1 * t85;
        let t88 = f64::powf(t76, 0.3473804e1);
        let t90 = 1.0 + 0.47702180224903349918e-3 * t88;
        let t91 = 1.0 / t90;
        let t93 = 0.60146019220211109872e-4 * t77 * t80 + t87 * t91;
        let t97 = piecewise3(t61, 0.0, -3.0 / 8.0 * t5 * t70 * t93);
        let tzk0 = t60 + t97;
        zk[ip] += tzk0;
    }
}
