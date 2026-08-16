//! LDA_XC_ZLP lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_zlp.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_XC_ZLP lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3::<f64>(rho[ip]);
        let t4 = 1.0 + 105.5562709925034 / t1;
        let t5 = f64::ln(t4);
        let t8 = 1.0 - 0.00947362 * t5 * t1;
        let t9 = t8 * t1;
        let tzk0 = -0.93222 * t9;
        zk[ip] += tzk0;
        let t12 = t1 * rho[ip];
        let t14 = 1.0 / t4;
        let t17 = t1 * t1;
        let t18 = 1.0 / t17;
        let t21 = 0.3333333333333333 / rho[ip] * t14 - 0.0031578733333333334 * t5 * t18;
        let tvrho0 = -1.24296 * t9 - 0.93222 * t12 * t21;
        vrho[ip] += tvrho0;
        let t28 = rho[ip] * rho[ip];
        let t34 = t4 * t4;
        let t35 = 1.0 / t34;
        let t39 = 1.0 / t17 / rho[ip];
        let t42 = -0.2222222222222222 / t28 * t14 + 11.728474554722599 / t1 / t28 * t35 + 0.002105248888888889 * t5 * t39;
        let tv2rho20 = -2.48592 * t21 * t1 - 0.41432 * t8 * t18 - 0.93222 * t12 * t42;
        v2rho2[ip] += tv2rho20;
        let t51 = t28 * rho[ip];
        let t60 = 1.0 / t17 / t51;
        let t62 = 1.0 / t34 / t4;
        let t66 = 1.0 / t17 / t28;
        let t69 = 0.37037037037037035 / t51 * t14 - 35.1854236641678 / t1 / t51 * t35 + 825.3426922846528 * t60 * t62 - 0.003508748148148148 * t5 * t66;
        let tv3rho30 = -3.72888 * t42 * t1 - 1.24296 * t21 * t18 + 0.2762133333333333 * t8 * t39 - 0.93222 * t12 * t69;
        v3rho3[ip] += tv3rho30;
        let t80 = t28 * t28;
        let t94 = t34 * t34;
        let tv4rho40 = -4.97184 * t69 * t1 - 2.48592 * t42 * t18 + 1.1048533333333332 * t21 * t39 - 0.4603555555555556 * t8 * t66 - 0.93222 * t12 * (-0.9876543209876543 / t80 * t14 + 130.3163839413622 / t1 / t80 * t35 - 5502.2846152310185 / t17 / t80 * t62 + 87120.09688848116 / t80 / rho[ip] / t94 + 0.009356661728395062 * t5 * t60);
        v4rho4[ip] += tv4rho40;
    }
}
