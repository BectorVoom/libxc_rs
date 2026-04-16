//! GGA_K_TFLW vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_tflw.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_tflw_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_gamma: f64,
    param_lambda: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t24 = param_lambda * sigma[ip];
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t28 = rho[ip] * rho[ip];
        let t31 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t36 = 1.0 / t35;
        let t40 = param_gamma + 5.0 / 72.0 * t24 * t26 / t22 / t28 * t31 * t36;
        let t44 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t40);
        let tzk0 = 2.0 * t44;
        zk[ip] += tzk0;
        let t50 = t28 * rho[ip];
        let t53 = t7 * t20 / t50;
        let t56 = t24 * t26 * t31 * t36;
        let t60 = piecewise3(t2, 0.0, t7 * t20 / t21 * t40 / 10.0 - t53 * t56 / 36.0);
        let tvrho0 = 2.0 * rho[ip] * t60 + 2.0 * t44;
        vrho[ip] += tvrho0;
        let t68 = param_lambda * t26 * t31 * t36;
        let t71 = piecewise3(t2, 0.0, t7 * t20 / t28 * t68 / 96.0);
        let tvsigma0 = 2.0 * rho[ip] * t71;
        vsigma[ip] += tvsigma0;
    }
}
