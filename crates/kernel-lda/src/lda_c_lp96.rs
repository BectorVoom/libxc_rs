//! LDA_C_LP96 kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_lp96.c`.
//! Translation preserves exact maple2c variable names and operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_LP96 exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t3 = param_C2 / t1;
        let t4 = t1 * t1;
        let t6 = param_C3 / t4;
        let tzk0 = param_C1 + t3 + t6;
        zk[ip] += tzk0;
    }
}

/// LDA_C_LP96 vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t3 = param_C2 / t1;
        let t4 = t1 * t1;
        let t6 = param_C3 / t4;
        let tzk0 = param_C1 + t3 + t6;
        zk[ip] += tzk0;
        let t9 = param_C2 / t1 / rho[ip];
        let t13 = param_C3 / t4 / rho[ip];
        let tvrho0 = param_C1 + t3 + t6 + rho[ip] * (-t9 / 3.0 - 2.0 / 3.0 * t13);
        vrho[ip] += tvrho0;
    }
}

/// LDA_C_LP96 fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t3 = param_C2 / t1;
        let t4 = t1 * t1;
        let t6 = param_C3 / t4;
        let tzk0 = param_C1 + t3 + t6;
        zk[ip] += tzk0;
        let t9 = param_C2 / t1 / rho[ip];
        let t13 = param_C3 / t4 / rho[ip];
        let tvrho0 = param_C1 + t3 + t6 + rho[ip] * (-t9 / 3.0 - 2.0 / 3.0 * t13);
        vrho[ip] += tvrho0;
        let t19 = rho[ip] * rho[ip];
        let t22 = param_C2 / t1 / t19;
        let t26 = param_C3 / t4 / t19;
        let tv2rho20 = -2.0 / 3.0 * t9 - 4.0 / 3.0 * t13 + rho[ip] * (4.0 / 9.0 * t22 + 10.0 / 9.0 * t26);
        v2rho2[ip] += tv2rho20;
    }
}

/// LDA_C_LP96 kxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t3 = param_C2 / t1;
        let t4 = t1 * t1;
        let t6 = param_C3 / t4;
        let tzk0 = param_C1 + t3 + t6;
        zk[ip] += tzk0;
        let t9 = param_C2 / t1 / rho[ip];
        let t13 = param_C3 / t4 / rho[ip];
        let tvrho0 = param_C1 + t3 + t6 + rho[ip] * (-t9 / 3.0 - 2.0 / 3.0 * t13);
        vrho[ip] += tvrho0;
        let t19 = rho[ip] * rho[ip];
        let t22 = param_C2 / t1 / t19;
        let t26 = param_C3 / t4 / t19;
        let tv2rho20 = -2.0 / 3.0 * t9 - 4.0 / 3.0 * t13 + rho[ip] * (4.0 / 9.0 * t22 + 10.0 / 9.0 * t26);
        v2rho2[ip] += tv2rho20;
        let t32 = t19 * rho[ip];
        let t35 = param_C2 / t1 / t32;
        let t39 = param_C3 / t4 / t32;
        let tv3rho30 = 4.0 / 3.0 * t22 + 10.0 / 3.0 * t26 + rho[ip] * (-28.0 / 27.0 * t35 - 80.0 / 27.0 * t39);
        v3rho3[ip] += tv3rho30;
    }
}

/// LDA_C_LP96 lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t3 = param_C2 / t1;
        let t4 = t1 * t1;
        let t6 = param_C3 / t4;
        let tzk0 = param_C1 + t3 + t6;
        zk[ip] += tzk0;
        let t9 = param_C2 / t1 / rho[ip];
        let t13 = param_C3 / t4 / rho[ip];
        let tvrho0 = param_C1 + t3 + t6 + rho[ip] * (-t9 / 3.0 - 2.0 / 3.0 * t13);
        vrho[ip] += tvrho0;
        let t19 = rho[ip] * rho[ip];
        let t22 = param_C2 / t1 / t19;
        let t26 = param_C3 / t4 / t19;
        let tv2rho20 = -2.0 / 3.0 * t9 - 4.0 / 3.0 * t13 + rho[ip] * (4.0 / 9.0 * t22 + 10.0 / 9.0 * t26);
        v2rho2[ip] += tv2rho20;
        let t32 = t19 * rho[ip];
        let t35 = param_C2 / t1 / t32;
        let t39 = param_C3 / t4 / t32;
        let tv3rho30 = 4.0 / 3.0 * t22 + 10.0 / 3.0 * t26 + rho[ip] * (-28.0 / 27.0 * t35 - 80.0 / 27.0 * t39);
        v3rho3[ip] += tv3rho30;
        let t45 = t19 * t19;
        let tv4rho40 = -112.0 / 27.0 * t35 - 320.0 / 27.0 * t39 + rho[ip] * (280.0 / 81.0 * param_C2 / t1 / t45 + 880.0 / 81.0 * param_C3 / t4 / t45);
        v4rho4[ip] += tv4rho40;
    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_LP96 exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t4 = param_C2 / t2;
        let t5 = t2 * t2;
        let t7 = param_C3 / t5;
        let tzk0 = param_C1 + t4 + t7;
        zk[ip] += tzk0;
    }
}

/// LDA_C_LP96 vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t4 = param_C2 / t2;
        let t5 = t2 * t2;
        let t7 = param_C3 / t5;
        let tzk0 = param_C1 + t4 + t7;
        zk[ip] += tzk0;
        let t10 = param_C2 / t2 / t1;
        let t14 = param_C3 / t5 / t1;
        let tvrho0 = param_C1 + t4 + t7 + t1 * (-t10 / 3.0 - 2.0 / 3.0 * t14);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}

/// LDA_C_LP96 fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t4 = param_C2 / t2;
        let t5 = t2 * t2;
        let t7 = param_C3 / t5;
        let tzk0 = param_C1 + t4 + t7;
        zk[ip] += tzk0;
        let t10 = param_C2 / t2 / t1;
        let t14 = param_C3 / t5 / t1;
        let tvrho0 = param_C1 + t4 + t7 + t1 * (-t10 / 3.0 - 2.0 / 3.0 * t14);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t20 = t1 * t1;
        let t23 = param_C2 / t2 / t20;
        let t27 = param_C3 / t5 / t20;
        let tv2rho20 = -2.0 / 3.0 * t10 - 4.0 / 3.0 * t14 + t1 * (4.0 / 9.0 * t23 + 10.0 / 9.0 * t27);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

/// LDA_C_LP96 kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t4 = param_C2 / t2;
        let t5 = t2 * t2;
        let t7 = param_C3 / t5;
        let tzk0 = param_C1 + t4 + t7;
        zk[ip] += tzk0;
        let t10 = param_C2 / t2 / t1;
        let t14 = param_C3 / t5 / t1;
        let tvrho0 = param_C1 + t4 + t7 + t1 * (-t10 / 3.0 - 2.0 / 3.0 * t14);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t20 = t1 * t1;
        let t23 = param_C2 / t2 / t20;
        let t27 = param_C3 / t5 / t20;
        let tv2rho20 = -2.0 / 3.0 * t10 - 4.0 / 3.0 * t14 + t1 * (4.0 / 9.0 * t23 + 10.0 / 9.0 * t27);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t33 = t20 * t1;
        let t36 = param_C2 / t2 / t33;
        let t40 = param_C3 / t5 / t33;
        let tv3rho30 = 4.0 / 3.0 * t23 + 10.0 / 3.0 * t27 + t1 * (-28.0 / 27.0 * t36 - 80.0 / 27.0 * t40);
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

/// LDA_C_LP96 lxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_lp96_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t4 = param_C2 / t2;
        let t5 = t2 * t2;
        let t7 = param_C3 / t5;
        let tzk0 = param_C1 + t4 + t7;
        zk[ip] += tzk0;
        let t10 = param_C2 / t2 / t1;
        let t14 = param_C3 / t5 / t1;
        let tvrho0 = param_C1 + t4 + t7 + t1 * (-t10 / 3.0 - 2.0 / 3.0 * t14);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t20 = t1 * t1;
        let t23 = param_C2 / t2 / t20;
        let t27 = param_C3 / t5 / t20;
        let tv2rho20 = -2.0 / 3.0 * t10 - 4.0 / 3.0 * t14 + t1 * (4.0 / 9.0 * t23 + 10.0 / 9.0 * t27);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t33 = t20 * t1;
        let t36 = param_C2 / t2 / t33;
        let t40 = param_C3 / t5 / t33;
        let tv3rho30 = 4.0 / 3.0 * t23 + 10.0 / 3.0 * t27 + t1 * (-28.0 / 27.0 * t36 - 80.0 / 27.0 * t40);
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t46 = t20 * t20;
        let tv4rho40 = -112.0 / 27.0 * t36 - 320.0 / 27.0 * t40 + t1 * (280.0 / 81.0 * param_C2 / t2 / t46 + 880.0 / 81.0 * param_C3 / t5 / t46);
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
