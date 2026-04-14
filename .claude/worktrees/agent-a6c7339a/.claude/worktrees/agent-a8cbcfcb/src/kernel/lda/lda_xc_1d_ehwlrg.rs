//! LDA_XC_1D_EHWLRG kernel functions translated from libxc maple2c.
//!
//! Auto-translated. Preserves exact maple2c variable names and operation order.
#![allow(clippy::excessive_precision, clippy::needless_return, unused_variables)]

use cubecl::prelude::*;

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_XC_1D_EHWLRG exc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip] * rho[ip];
        let t4 = param_a2 * rho[ip] + param_a3 * t1 + param_a1;
        let t5 = f64::powf(rho[ip], param_alpha);
        let tzk0 = t4 * t5;
        zk[ip] += tzk0;
    }
}

/// LDA_XC_1D_EHWLRG vxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip] * rho[ip];
        let t4 = param_a2 * rho[ip] + param_a3 * t1 + param_a1;
        let t5 = f64::powf(rho[ip], param_alpha);
        let tzk0 = t4 * t5;
        zk[ip] += tzk0;
        let t6 = rho[ip] * param_a3;
        let t8 = 2.0 * t6 + param_a2;
        let tvrho0 = rho[ip] * t8 * t5 + t4 * t5 * param_alpha + tzk0;
        vrho[ip] += tvrho0;
    }
}

/// LDA_XC_1D_EHWLRG fxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip] * rho[ip];
        let t4 = param_a2 * rho[ip] + param_a3 * t1 + param_a1;
        let t5 = f64::powf(rho[ip], param_alpha);
        let tzk0 = t4 * t5;
        zk[ip] += tzk0;
        let t6 = rho[ip] * param_a3;
        let t8 = 2.0 * t6 + param_a2;
        let tvrho0 = rho[ip] * t8 * t5 + t4 * t5 * param_alpha + tzk0;
        vrho[ip] += tvrho0;
        let t13 = t8 * t5;
        let t15 = 1.0 / rho[ip];
        let t16 = param_alpha * t15;
        let t22 = param_alpha * param_alpha;
        let t23 = t22 * t15;
        let tv2rho20 = 2.0 * t13 * param_alpha + tzk0 * t16 + tzk0 * t23 + 2.0 * t6 * t5 + 2.0 * t13;
        v2rho2[ip] += tv2rho20;
    }
}

/// LDA_XC_1D_EHWLRG kxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip] * rho[ip];
        let t4 = param_a2 * rho[ip] + param_a3 * t1 + param_a1;
        let t5 = f64::powf(rho[ip], param_alpha);
        let tzk0 = t4 * t5;
        zk[ip] += tzk0;
        let t6 = rho[ip] * param_a3;
        let t8 = 2.0 * t6 + param_a2;
        let tvrho0 = rho[ip] * t8 * t5 + t4 * t5 * param_alpha + tzk0;
        vrho[ip] += tvrho0;
        let t13 = t8 * t5;
        let t15 = 1.0 / rho[ip];
        let t16 = param_alpha * t15;
        let t22 = param_alpha * param_alpha;
        let t23 = t22 * t15;
        let tv2rho20 = 2.0 * t13 * param_alpha + tzk0 * t16 + tzk0 * t23 + 2.0 * t6 * t5 + 2.0 * t13;
        v2rho2[ip] += tv2rho20;
        let t25 = param_a3 * t5;
        let t29 = 1.0 / t1;
        let t30 = param_alpha * t29;
        let t36 = t22 * param_alpha;
        let t37 = t36 * t29;
        let tv3rho30 = 3.0 * t13 * t16 + 3.0 * t13 * t23 + 6.0 * t25 * param_alpha - tzk0 * t30 + tzk0 * t37 + 6.0 * t25;
        v3rho3[ip] += tv3rho30;
    }
}

/// LDA_XC_1D_EHWLRG lxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip] * rho[ip];
        let t4 = param_a2 * rho[ip] + param_a3 * t1 + param_a1;
        let t5 = f64::powf(rho[ip], param_alpha);
        let tzk0 = t4 * t5;
        zk[ip] += tzk0;
        let t6 = rho[ip] * param_a3;
        let t8 = 2.0 * t6 + param_a2;
        let tvrho0 = rho[ip] * t8 * t5 + t4 * t5 * param_alpha + tzk0;
        vrho[ip] += tvrho0;
        let t13 = t8 * t5;
        let t15 = 1.0 / rho[ip];
        let t16 = param_alpha * t15;
        let t22 = param_alpha * param_alpha;
        let t23 = t22 * t15;
        let tv2rho20 = 2.0 * t13 * param_alpha + tzk0 * t16 + tzk0 * t23 + 2.0 * t6 * t5 + 2.0 * t13;
        v2rho2[ip] += tv2rho20;
        let t25 = param_a3 * t5;
        let t29 = 1.0 / t1;
        let t30 = param_alpha * t29;
        let t36 = t22 * param_alpha;
        let t37 = t36 * t29;
        let tv3rho30 = 3.0 * t13 * t16 + 3.0 * t13 * t23 + 6.0 * t25 * param_alpha - tzk0 * t30 + tzk0 * t37 + 6.0 * t25;
        v3rho3[ip] += tv3rho30;
        let t44 = 1.0 / t1 / rho[ip];
        let t54 = t22 * t22;
        let tv4rho40 = -tzk0 * t22 * t44 - 2.0 * tzk0 * t36 * t44 + tzk0 * t54 * t44 + 2.0 * tzk0 * param_alpha * t44 - 4.0 * t13 * t30 + 4.0 * t13 * t37 + 12.0 * t25 * t16 + 12.0 * t25 * t23;
        v4rho4[ip] += tv4rho40;
    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_XC_1D_EHWLRG exc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = rho0 + rho1;
        let t3 = t1 * t1;
        let t5 = param_a2 * t1 + param_a3 * t3 + param_a1;
        let t6 = f64::powf(t1, param_alpha);
        let tzk0 = t5 * t6;
        zk[ip] += tzk0;
    }
}

/// LDA_XC_1D_EHWLRG vxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = rho0 + rho1;
        let t3 = t1 * t1;
        let t5 = param_a2 * t1 + param_a3 * t3 + param_a1;
        let t6 = f64::powf(t1, param_alpha);
        let tzk0 = t5 * t6;
        zk[ip] += tzk0;
        let t7 = param_a3 * t1;
        let t9 = param_a2 + 2.0 * t7;
        let tvrho0 = t1 * t9 * t6 + t5 * t6 * param_alpha + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}

/// LDA_XC_1D_EHWLRG fxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = rho0 + rho1;
        let t3 = t1 * t1;
        let t5 = param_a2 * t1 + param_a3 * t3 + param_a1;
        let t6 = f64::powf(t1, param_alpha);
        let tzk0 = t5 * t6;
        zk[ip] += tzk0;
        let t7 = param_a3 * t1;
        let t9 = param_a2 + 2.0 * t7;
        let tvrho0 = t1 * t9 * t6 + t5 * t6 * param_alpha + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t14 = t9 * t6;
        let t16 = 1.0 / t1;
        let t17 = param_alpha * t16;
        let t23 = param_alpha * param_alpha;
        let t24 = t23 * t16;
        let tv2rho20 = 2.0 * t14 * param_alpha + tzk0 * t17 + tzk0 * t24 + 2.0 * t7 * t6 + 2.0 * t14;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

/// LDA_XC_1D_EHWLRG kxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = rho0 + rho1;
        let t3 = t1 * t1;
        let t5 = param_a2 * t1 + param_a3 * t3 + param_a1;
        let t6 = f64::powf(t1, param_alpha);
        let tzk0 = t5 * t6;
        zk[ip] += tzk0;
        let t7 = param_a3 * t1;
        let t9 = param_a2 + 2.0 * t7;
        let tvrho0 = t1 * t9 * t6 + t5 * t6 * param_alpha + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t14 = t9 * t6;
        let t16 = 1.0 / t1;
        let t17 = param_alpha * t16;
        let t23 = param_alpha * param_alpha;
        let t24 = t23 * t16;
        let tv2rho20 = 2.0 * t14 * param_alpha + tzk0 * t17 + tzk0 * t24 + 2.0 * t7 * t6 + 2.0 * t14;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t26 = param_a3 * t6;
        let t30 = 1.0 / t3;
        let t31 = param_alpha * t30;
        let t37 = t23 * param_alpha;
        let t38 = t37 * t30;
        let tv3rho30 = 3.0 * t14 * t17 + 3.0 * t14 * t24 + 6.0 * t26 * param_alpha - tzk0 * t31 + tzk0 * t38 + 6.0 * t26;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

/// LDA_XC_1D_EHWLRG lxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = rho0 + rho1;
        let t3 = t1 * t1;
        let t5 = param_a2 * t1 + param_a3 * t3 + param_a1;
        let t6 = f64::powf(t1, param_alpha);
        let tzk0 = t5 * t6;
        zk[ip] += tzk0;
        let t7 = param_a3 * t1;
        let t9 = param_a2 + 2.0 * t7;
        let tvrho0 = t1 * t9 * t6 + t5 * t6 * param_alpha + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t14 = t9 * t6;
        let t16 = 1.0 / t1;
        let t17 = param_alpha * t16;
        let t23 = param_alpha * param_alpha;
        let t24 = t23 * t16;
        let tv2rho20 = 2.0 * t14 * param_alpha + tzk0 * t17 + tzk0 * t24 + 2.0 * t7 * t6 + 2.0 * t14;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t26 = param_a3 * t6;
        let t30 = 1.0 / t3;
        let t31 = param_alpha * t30;
        let t37 = t23 * param_alpha;
        let t38 = t37 * t30;
        let tv3rho30 = 3.0 * t14 * t17 + 3.0 * t14 * t24 + 6.0 * t26 * param_alpha - tzk0 * t31 + tzk0 * t38 + 6.0 * t26;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t45 = 1.0 / t3 / t1;
        let t55 = t23 * t23;
        let tv4rho40 = -tzk0 * t23 * t45 - 2.0 * tzk0 * t37 * t45 + tzk0 * t55 * t45 + 2.0 * tzk0 * param_alpha * t45 - 4.0 * t14 * t31 + 4.0 * t14 * t38 + 12.0 * t26 * t17 + 12.0 * t26 * t24;
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
