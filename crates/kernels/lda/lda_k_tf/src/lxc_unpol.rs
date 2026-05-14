//! LDA_K_TF lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_tf.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_K_TF lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_tf_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_ax: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = pow_1_3(zeta_threshold);
        let t3 = t2 * t2;
        let t5 = piecewise3(1.0 <= zeta_threshold, t3 * zeta_threshold, 1.0);
        let t7 = M_CBRT3;
        let t8 = param_ax * t5 * t7;
        let t10 = pow_1_3(1.0 / M_PI);
        let t11 = t10 * t10;
        let t13 = M_CBRT4;
        let t14 = t13 * t13;
        let t15 = 1.0 / t11 * t14;
        let t16 = pow_1_3(rho[ip]);
        let t17 = t16 * t16;
        let t19 = t8 * t15 * t17;
        let tzk0 = t19 / 3.0;
        zk[ip] += tzk0;
        let tvrho0 = 5.0 / 9.0 * t19;
        vrho[ip] += tvrho0;
        let tv2rho20 = 10.0 / 27.0 * t8 * t15 / t16;
        v2rho2[ip] += tv2rho20;
        let tv3rho30 = -10.0 / 81.0 * t8 * t15 / t16 / rho[ip];
        v3rho3[ip] += tv3rho30;
        let t28 = rho[ip] * rho[ip];
        let tv4rho40 = 40.0 / 243.0 * t8 * t15 / t16 / t28;
        v4rho4[ip] += tv4rho40;
    }
}
