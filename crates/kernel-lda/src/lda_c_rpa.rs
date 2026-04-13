//! LDA_C_RPA kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rpa.c`.
//! Translation preserves exact maple2c variable names and operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_RPA exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t12 = f64::ln(t10 / 4.0);
        let t13 = 0.0311 * t12;
        let t16 = 0.00225 * t4 * t9 * t12;
        let t17 = 0.00425 * t10;
        let tzk0 = t13 - 0.048 + t16 - t17;
        zk[ip] += tzk0;
    }
}

/// LDA_C_RPA vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t12 = f64::ln(t10 / 4.0);
        let t13 = 0.0311 * t12;
        let t16 = 0.00225 * t4 * t9 * t12;
        let t17 = 0.00425 * t10;
        let tzk0 = t13 - 0.048 + t16 - t17;
        zk[ip] += tzk0;
        let t18 = 1.0 / rho[ip];
        let t22 = t6 / t7 / rho[ip];
        let t24 = t4 * t22 * t12;
        let t26 = t4 * t22;
        let tvrho0 = t13 - 0.048 + t16 - t17 + rho[ip] * (-0.010366666666666666 * t18 - 0.00075 * t24 + 0.0006666666666666666 * t26);
        vrho[ip] += tvrho0;
    }
}

/// LDA_C_RPA fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t12 = f64::ln(t10 / 4.0);
        let t13 = 0.0311 * t12;
        let t16 = 0.00225 * t4 * t9 * t12;
        let t17 = 0.00425 * t10;
        let tzk0 = t13 - 0.048 + t16 - t17;
        zk[ip] += tzk0;
        let t18 = 1.0 / rho[ip];
        let t22 = t6 / t7 / rho[ip];
        let t24 = t4 * t22 * t12;
        let t26 = t4 * t22;
        let tvrho0 = t13 - 0.048 + t16 - t17 + rho[ip] * (-0.010366666666666666 * t18 - 0.00075 * t24 + 0.0006666666666666666 * t26);
        vrho[ip] += tvrho0;
        let t33 = rho[ip] * rho[ip];
        let t34 = 1.0 / t33;
        let t38 = t6 / t7 / t33;
        let t40 = t4 * t38 * t12;
        let t42 = t4 * t38;
        let tv2rho20 = -0.020733333333333333 * t18 - 0.0015 * t24 + 0.0013333333333333333 * t26 + rho[ip] * (0.010366666666666666 * t34 + 0.001 * t40 - 0.0006388888888888889 * t42);
        v2rho2[ip] += tv2rho20;
    }
}

/// LDA_C_RPA kxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_kxc_unpol(
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
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t12 = f64::ln(t10 / 4.0);
        let t13 = 0.0311 * t12;
        let t16 = 0.00225 * t4 * t9 * t12;
        let t17 = 0.00425 * t10;
        let tzk0 = t13 - 0.048 + t16 - t17;
        zk[ip] += tzk0;
        let t18 = 1.0 / rho[ip];
        let t22 = t6 / t7 / rho[ip];
        let t24 = t4 * t22 * t12;
        let t26 = t4 * t22;
        let tvrho0 = t13 - 0.048 + t16 - t17 + rho[ip] * (-0.010366666666666666 * t18 - 0.00075 * t24 + 0.0006666666666666666 * t26);
        vrho[ip] += tvrho0;
        let t33 = rho[ip] * rho[ip];
        let t34 = 1.0 / t33;
        let t38 = t6 / t7 / t33;
        let t40 = t4 * t38 * t12;
        let t42 = t4 * t38;
        let tv2rho20 = -0.020733333333333333 * t18 - 0.0015 * t24 + 0.0013333333333333333 * t26 + rho[ip] * (0.010366666666666666 * t34 + 0.001 * t40 - 0.0006388888888888889 * t42);
        v2rho2[ip] += tv2rho20;
        let t49 = t33 * rho[ip];
        let t50 = 1.0 / t49;
        let t54 = t6 / t7 / t49;
        let t56 = t4 * t54 * t12;
        let t58 = t4 * t54;
        let tv3rho30 = 0.0311 * t34 + 0.003 * t40 - 0.0019166666666666666 * t42 + rho[ip] * (-0.020733333333333333 * t50 - 0.0023333333333333335 * t56 + 0.0011574074074074073 * t58);
        v3rho3[ip] += tv3rho30;
    }
}

/// LDA_C_RPA lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_lxc_unpol(
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
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t12 = f64::ln(t10 / 4.0);
        let t13 = 0.0311 * t12;
        let t16 = 0.00225 * t4 * t9 * t12;
        let t17 = 0.00425 * t10;
        let tzk0 = t13 - 0.048 + t16 - t17;
        zk[ip] += tzk0;
        let t18 = 1.0 / rho[ip];
        let t22 = t6 / t7 / rho[ip];
        let t24 = t4 * t22 * t12;
        let t26 = t4 * t22;
        let tvrho0 = t13 - 0.048 + t16 - t17 + rho[ip] * (-0.010366666666666666 * t18 - 0.00075 * t24 + 0.0006666666666666666 * t26);
        vrho[ip] += tvrho0;
        let t33 = rho[ip] * rho[ip];
        let t34 = 1.0 / t33;
        let t38 = t6 / t7 / t33;
        let t40 = t4 * t38 * t12;
        let t42 = t4 * t38;
        let tv2rho20 = -0.020733333333333333 * t18 - 0.0015 * t24 + 0.0013333333333333333 * t26 + rho[ip] * (0.010366666666666666 * t34 + 0.001 * t40 - 0.0006388888888888889 * t42);
        v2rho2[ip] += tv2rho20;
        let t49 = t33 * rho[ip];
        let t50 = 1.0 / t49;
        let t54 = t6 / t7 / t49;
        let t56 = t4 * t54 * t12;
        let t58 = t4 * t54;
        let tv3rho30 = 0.0311 * t34 + 0.003 * t40 - 0.0019166666666666666 * t42 + rho[ip] * (-0.020733333333333333 * t50 - 0.0023333333333333335 * t56 + 0.0011574074074074073 * t58);
        v3rho3[ip] += tv3rho30;
        let t65 = t33 * t33;
        let t70 = t6 / t7 / t65;
        let tv4rho40 = -0.08293333333333333 * t50 - 0.009333333333333334 * t56 + 0.004629629629629629 * t58 + rho[ip] * (0.0622 / t65 + 0.0077777777777777776 * t4 * t70 * t12 - 0.003080246913580247 * t4 * t70);
        v4rho4[ip] += tv4rho40;
    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_RPA exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t13 = f64::ln(t11 / 4.0);
        let t14 = 0.0311 * t13;
        let t17 = 0.00225 * t4 * t10 * t13;
        let t18 = 0.00425 * t11;
        let tzk0 = t14 - 0.048 + t17 - t18;
        zk[ip] += tzk0;
    }
}

/// LDA_C_RPA vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_vxc_pol(
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
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t13 = f64::ln(t11 / 4.0);
        let t14 = 0.0311 * t13;
        let t17 = 0.00225 * t4 * t10 * t13;
        let t18 = 0.00425 * t11;
        let tzk0 = t14 - 0.048 + t17 - t18;
        zk[ip] += tzk0;
        let t19 = 1.0 / t7;
        let t23 = t6 / t8 / t7;
        let t25 = t4 * t23 * t13;
        let t27 = t4 * t23;
        let tvrho0 = t14 - 0.048 + t17 - t18 + t7 * (-0.010366666666666666 * t19 - 0.00075 * t25 + 0.0006666666666666666 * t27);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}

/// LDA_C_RPA fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_fxc_pol(
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
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t13 = f64::ln(t11 / 4.0);
        let t14 = 0.0311 * t13;
        let t17 = 0.00225 * t4 * t10 * t13;
        let t18 = 0.00425 * t11;
        let tzk0 = t14 - 0.048 + t17 - t18;
        zk[ip] += tzk0;
        let t19 = 1.0 / t7;
        let t23 = t6 / t8 / t7;
        let t25 = t4 * t23 * t13;
        let t27 = t4 * t23;
        let tvrho0 = t14 - 0.048 + t17 - t18 + t7 * (-0.010366666666666666 * t19 - 0.00075 * t25 + 0.0006666666666666666 * t27);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t34 = t7 * t7;
        let t35 = 1.0 / t34;
        let t39 = t6 / t8 / t34;
        let t41 = t4 * t39 * t13;
        let t43 = t4 * t39;
        let tv2rho20 = -0.020733333333333333 * t19 - 0.0015 * t25 + 0.0013333333333333333 * t27 + t7 * (0.010366666666666666 * t35 + 0.001 * t41 - 0.0006388888888888889 * t43);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

/// LDA_C_RPA kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_kxc_pol(
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
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t13 = f64::ln(t11 / 4.0);
        let t14 = 0.0311 * t13;
        let t17 = 0.00225 * t4 * t10 * t13;
        let t18 = 0.00425 * t11;
        let tzk0 = t14 - 0.048 + t17 - t18;
        zk[ip] += tzk0;
        let t19 = 1.0 / t7;
        let t23 = t6 / t8 / t7;
        let t25 = t4 * t23 * t13;
        let t27 = t4 * t23;
        let tvrho0 = t14 - 0.048 + t17 - t18 + t7 * (-0.010366666666666666 * t19 - 0.00075 * t25 + 0.0006666666666666666 * t27);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t34 = t7 * t7;
        let t35 = 1.0 / t34;
        let t39 = t6 / t8 / t34;
        let t41 = t4 * t39 * t13;
        let t43 = t4 * t39;
        let tv2rho20 = -0.020733333333333333 * t19 - 0.0015 * t25 + 0.0013333333333333333 * t27 + t7 * (0.010366666666666666 * t35 + 0.001 * t41 - 0.0006388888888888889 * t43);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t50 = t34 * t7;
        let t51 = 1.0 / t50;
        let t55 = t6 / t8 / t50;
        let t57 = t4 * t55 * t13;
        let t59 = t4 * t55;
        let tv3rho30 = 0.0311 * t35 + 0.003 * t41 - 0.0019166666666666666 * t43 + t7 * (-0.020733333333333333 * t51 - 0.0023333333333333335 * t57 + 0.0011574074074074073 * t59);
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

/// LDA_C_RPA lxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_lxc_pol(
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
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t13 = f64::ln(t11 / 4.0);
        let t14 = 0.0311 * t13;
        let t17 = 0.00225 * t4 * t10 * t13;
        let t18 = 0.00425 * t11;
        let tzk0 = t14 - 0.048 + t17 - t18;
        zk[ip] += tzk0;
        let t19 = 1.0 / t7;
        let t23 = t6 / t8 / t7;
        let t25 = t4 * t23 * t13;
        let t27 = t4 * t23;
        let tvrho0 = t14 - 0.048 + t17 - t18 + t7 * (-0.010366666666666666 * t19 - 0.00075 * t25 + 0.0006666666666666666 * t27);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t34 = t7 * t7;
        let t35 = 1.0 / t34;
        let t39 = t6 / t8 / t34;
        let t41 = t4 * t39 * t13;
        let t43 = t4 * t39;
        let tv2rho20 = -0.020733333333333333 * t19 - 0.0015 * t25 + 0.0013333333333333333 * t27 + t7 * (0.010366666666666666 * t35 + 0.001 * t41 - 0.0006388888888888889 * t43);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t50 = t34 * t7;
        let t51 = 1.0 / t50;
        let t55 = t6 / t8 / t50;
        let t57 = t4 * t55 * t13;
        let t59 = t4 * t55;
        let tv3rho30 = 0.0311 * t35 + 0.003 * t41 - 0.0019166666666666666 * t43 + t7 * (-0.020733333333333333 * t51 - 0.0023333333333333335 * t57 + 0.0011574074074074073 * t59);
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t66 = t34 * t34;
        let t71 = t6 / t8 / t66;
        let tv4rho40 = -0.08293333333333333 * t51 - 0.009333333333333334 * t57 + 0.004629629629629629 * t59 + t7 * (0.0622 / t66 + 0.0077777777777777776 * t4 * t71 * t13 - 0.003080246913580247 * t4 * t71);
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
