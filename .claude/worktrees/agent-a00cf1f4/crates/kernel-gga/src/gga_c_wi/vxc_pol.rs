//! GGA_C_WI vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wi.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_wi_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_k: f64,
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
        let t2 = sigma0 + 2.0 * sigma1 + sigma2;
        let t3 = param_b * t2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = pow_1_3(t4);
        let t7 = t6 * t6;
        let t9 = 1.0 / t7 / t5;
        let t10 = param_k * t2;
        let t12 = f64::exp(-t10 * t9);
        let t15 = t3 * t9 * t12 + param_a;
        let t16 = M_CBRT3;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = t16 * t18;
        let t20 = M_CBRT4;
        let t21 = t20 * t20;
        let t25 = t16 * t16;
        let t26 = M_CBRTPI;
        let t28 = f64::sqrt(t2);
        let t29 = t28 * t2;
        let t30 = t5 * t5;
        let t31 = 1.0 / t30;
        let t34 = 1.0 / t6 / t4;
        let t35 = t28 * t34;
        let t36 = f64::sqrt(t35);
        let t41 = 1.0 + param_d * t20 * t25 * t26 * t36 * t29 * t31 / 3.0;
        let t45 = param_c + t19 * t21 / t6 * t41 / 4.0;
        let t46 = 1.0 / t45;
        let tzk0 = t15 * t46;
        zk[ip] += tzk0;
        let t47 = t5 * t4;
        let t49 = 1.0 / t7 / t47;
        let t52 = t2 * t2;
        let t53 = param_b * t52;
        let t54 = t30 * t5;
        let t56 = 1.0 / t6 / t54;
        let t61 = 8.0 / 3.0 * t53 * t56 * param_k * t12 - 8.0 / 3.0 * t3 * t49 * t12;
        let t62 = t4 * t61;
        let t64 = t4 * t15;
        let t65 = t45 * t45;
        let t66 = 1.0 / t65;
        let t74 = t36 * t2 * t9;
        let t75 = t26 * t74;
        let t76 = t75 * t28;
        let t79 = -t19 * t21 * t34 * t41 / 12.0 - 14.0 / 3.0 * t18 * t9 * param_d * t76;
        let t80 = t66 * t79;
        let tvrho0 = t62 * t46 - t64 * t80 + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t84 = t30 * t4;
        let t86 = 1.0 / t6 / t84;
        let t90 = -t3 * t86 * param_k * t12 + param_b * t9 * t12;
        let t91 = t4 * t90;
        let t93 = 1.0 / t7;
        let t94 = t93 * t15;
        let t95 = t66 * t18;
        let t96 = t94 * t95;
        let t97 = param_d * t26;
        let t98 = 1.0 / t28;
        let t99 = t74 * t98;
        let t100 = t97 * t99;
        let t101 = t96 * t100;
        let tvsigma0 = t91 * t46 - 7.0 / 4.0 * t101;
        vsigma[ip * 3] += tvsigma0;
        let t103 = 2.0 * t90;
        let t104 = t4 * t103;
        let tvsigma1 = t104 * t46 - 7.0 / 2.0 * t101;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
