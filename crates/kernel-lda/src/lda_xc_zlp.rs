//! LDA_XC_ZLP kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_zlp.c`.
//! Translation preserves exact maple2c variable names and operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_XC_ZLP exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t4 = 1.0 + 105.5562709925034 / t1;
        let t5 = f64::ln(t4);
        let t8 = 1.0 - 0.00947362 * t5 * t1;
        let t9 = t8 * t1;
        let tzk0 = -0.93222 * t9;
        zk[ip] += tzk0;
    }
}

/// LDA_XC_ZLP vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
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
    }
}

/// LDA_XC_ZLP fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
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
    }
}

/// LDA_XC_ZLP kxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
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
    }
}

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
        let t1 = pow_1_3(rho[ip]);
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

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_XC_ZLP exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.5562709925034 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        zk[ip] += tzk0;
    }
}

/// LDA_XC_ZLP vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.5562709925034 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        zk[ip] += tzk0;
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.3333333333333333 / t1 * t15 - 0.0031578733333333334 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}

/// LDA_XC_ZLP fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.5562709925034 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        zk[ip] += tzk0;
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.3333333333333333 / t1 * t15 - 0.0031578733333333334 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t29 = t1 * t1;
        let t35 = t5 * t5;
        let t36 = 1.0 / t35;
        let t40 = 1.0 / t18 / t1;
        let t43 = -0.2222222222222222 / t29 * t15 + 11.728474554722599 / t2 / t29 * t36 + 0.002105248888888889 * t6 * t40;
        let tv2rho20 = -2.48592 * t22 * t2 - 0.41432 * t9 * t19 - 0.93222 * t13 * t43;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

/// LDA_XC_ZLP kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.5562709925034 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        zk[ip] += tzk0;
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.3333333333333333 / t1 * t15 - 0.0031578733333333334 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t29 = t1 * t1;
        let t35 = t5 * t5;
        let t36 = 1.0 / t35;
        let t40 = 1.0 / t18 / t1;
        let t43 = -0.2222222222222222 / t29 * t15 + 11.728474554722599 / t2 / t29 * t36 + 0.002105248888888889 * t6 * t40;
        let tv2rho20 = -2.48592 * t22 * t2 - 0.41432 * t9 * t19 - 0.93222 * t13 * t43;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t52 = t29 * t1;
        let t61 = 1.0 / t18 / t52;
        let t63 = 1.0 / t35 / t5;
        let t67 = 1.0 / t18 / t29;
        let t70 = 0.37037037037037035 / t52 * t15 - 35.1854236641678 / t2 / t52 * t36 + 825.3426922846528 * t61 * t63 - 0.003508748148148148 * t6 * t67;
        let tv3rho30 = -3.72888 * t43 * t2 - 1.24296 * t22 * t19 + 0.2762133333333333 * t9 * t40 - 0.93222 * t13 * t70;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

/// LDA_XC_ZLP lxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_lxc_pol(
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
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.5562709925034 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        zk[ip] += tzk0;
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.3333333333333333 / t1 * t15 - 0.0031578733333333334 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t29 = t1 * t1;
        let t35 = t5 * t5;
        let t36 = 1.0 / t35;
        let t40 = 1.0 / t18 / t1;
        let t43 = -0.2222222222222222 / t29 * t15 + 11.728474554722599 / t2 / t29 * t36 + 0.002105248888888889 * t6 * t40;
        let tv2rho20 = -2.48592 * t22 * t2 - 0.41432 * t9 * t19 - 0.93222 * t13 * t43;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t52 = t29 * t1;
        let t61 = 1.0 / t18 / t52;
        let t63 = 1.0 / t35 / t5;
        let t67 = 1.0 / t18 / t29;
        let t70 = 0.37037037037037035 / t52 * t15 - 35.1854236641678 / t2 / t52 * t36 + 825.3426922846528 * t61 * t63 - 0.003508748148148148 * t6 * t67;
        let tv3rho30 = -3.72888 * t43 * t2 - 1.24296 * t22 * t19 + 0.2762133333333333 * t9 * t40 - 0.93222 * t13 * t70;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t81 = t29 * t29;
        let t95 = t35 * t35;
        let tv4rho40 = -4.97184 * t70 * t2 - 2.48592 * t43 * t19 + 1.1048533333333332 * t22 * t40 - 0.4603555555555556 * t9 * t67 - 0.93222 * t13 * (-0.9876543209876543 / t81 * t15 + 130.3163839413622 / t2 / t81 * t36 - 5502.2846152310185 / t18 / t81 * t63 + 87120.09688848116 / t81 / t1 / t95 + 0.009356661728395062 * t6 * t61);
        v4rho4[ip * 5] += tv4rho40;
        let tv4rho41 = tv4rho40;
        v4rho4[ip * 5 + 1] += tv4rho41;
        let tv4rho42 = tv4rho41;
        v4rho4[ip * 5 + 2] += tv4rho42;
        let tv4rho43 = tv4rho42;
        v4rho4[ip * 5 + 3] += tv4rho43;
        let tv4rho44 = tv4rho43;
        v4rho4[ip * 5 + 4] += tv4rho44;
    }
}
