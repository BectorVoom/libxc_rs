//! LDA XC ZLP kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_zlp.c`.

use cubecl::prelude::*;
use crate::math::powers::{pow_1_3};

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t4 = 1.0 + 105.556270992503 / t1;
        let t5 = f64::ln(t4);
        let t8 = 1.0 - 0.00947362 * t5 * t1;
        let t9 = t8 * t1;
        let tzk0 = -0.93222 * t9;
        zk[ip] += tzk0;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t4 = 1.0 + 105.556270992503 / t1;
        let t5 = f64::ln(t4);
        let t8 = 1.0 - 0.00947362 * t5 * t1;
        let t9 = t8 * t1;
        let tzk0 = -0.93222 * t9;
        let t12 = t1 * rho[ip];
        let t14 = 1.0 / t4;
        let t17 = t1 * t1;
        let t18 = 1.0 / t17;
        let t21 = 0.333333333333333 / rho[ip] * t14 - 0.00315787333333333 * t5 * t18;
        let tvrho0 = -1.24296 * t9 - 0.93222 * t12 * t21;
        zk[ip] += tzk0;
        vrho[ip] += tvrho0;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t4 = 1.0 + 105.556270992503 / t1;
        let t5 = f64::ln(t4);
        let t8 = 1.0 - 0.00947362 * t5 * t1;
        let t9 = t8 * t1;
        let tzk0 = -0.93222 * t9;
        let t12 = t1 * rho[ip];
        let t14 = 1.0 / t4;
        let t17 = t1 * t1;
        let t18 = 1.0 / t17;
        let t21 = 0.333333333333333 / rho[ip] * t14 - 0.00315787333333333 * t5 * t18;
        let tvrho0 = -1.24296 * t9 - 0.93222 * t12 * t21;
        let t28 = rho[ip] * rho[ip];
        let t34 = t4 * t4;
        let t35 = 1.0 / t34;
        let t39 = 1.0 / t17 / rho[ip];
        let t42 = -0.222222222222222 / t28 * t14 + 11.7284745547226 / t1 / t28 * t35 + 0.00210524888888889 * t5 * t39;
        let tv2rho20 = -2.48592 * t21 * t1 - 0.41432 * t8 * t18 - 0.93222 * t12 * t42;
        zk[ip] += tzk0;
        vrho[ip] += tvrho0;
        v2rho2[ip] += tv2rho20;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t4 = 1.0 + 105.556270992503 / t1;
        let t5 = f64::ln(t4);
        let t8 = 1.0 - 0.00947362 * t5 * t1;
        let t9 = t8 * t1;
        let tzk0 = -0.93222 * t9;
        let t12 = t1 * rho[ip];
        let t14 = 1.0 / t4;
        let t17 = t1 * t1;
        let t18 = 1.0 / t17;
        let t21 = 0.333333333333333 / rho[ip] * t14 - 0.00315787333333333 * t5 * t18;
        let tvrho0 = -1.24296 * t9 - 0.93222 * t12 * t21;
        let t28 = rho[ip] * rho[ip];
        let t34 = t4 * t4;
        let t35 = 1.0 / t34;
        let t39 = 1.0 / t17 / rho[ip];
        let t42 = -0.222222222222222 / t28 * t14 + 11.7284745547226 / t1 / t28 * t35 + 0.00210524888888889 * t5 * t39;
        let tv2rho20 = -2.48592 * t21 * t1 - 0.41432 * t8 * t18 - 0.93222 * t12 * t42;
        let t51 = t28 * rho[ip];
        let t60 = 1.0 / t17 / t51;
        let t62 = 1.0 / t34 / t4;
        let t66 = 1.0 / t17 / t28;
        let t69 = 0.37037037037037 / t51 * t14 - 35.1854236641678 / t1 / t51 * t35 + 825.342692284653 * t60 * t62 - 0.00350874814814815 * t5 * t66;
        let tv3rho30 = -3.72888 * t42 * t1 - 1.24296 * t21 * t18 + 0.276213333333333 * t8 * t39 - 0.93222 * t12 * t69;
        zk[ip] += tzk0;
        vrho[ip] += tvrho0;
        v2rho2[ip] += tv2rho20;
        v3rho3[ip] += tv3rho30;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t4 = 1.0 + 105.556270992503 / t1;
        let t5 = f64::ln(t4);
        let t8 = 1.0 - 0.00947362 * t5 * t1;
        let t9 = t8 * t1;
        let tzk0 = -0.93222 * t9;
        let t12 = t1 * rho[ip];
        let t14 = 1.0 / t4;
        let t17 = t1 * t1;
        let t18 = 1.0 / t17;
        let t21 = 0.333333333333333 / rho[ip] * t14 - 0.00315787333333333 * t5 * t18;
        let tvrho0 = -1.24296 * t9 - 0.93222 * t12 * t21;
        let t28 = rho[ip] * rho[ip];
        let t34 = t4 * t4;
        let t35 = 1.0 / t34;
        let t39 = 1.0 / t17 / rho[ip];
        let t42 = -0.222222222222222 / t28 * t14 + 11.7284745547226 / t1 / t28 * t35 + 0.00210524888888889 * t5 * t39;
        let tv2rho20 = -2.48592 * t21 * t1 - 0.41432 * t8 * t18 - 0.93222 * t12 * t42;
        let t51 = t28 * rho[ip];
        let t60 = 1.0 / t17 / t51;
        let t62 = 1.0 / t34 / t4;
        let t66 = 1.0 / t17 / t28;
        let t69 = 0.37037037037037 / t51 * t14 - 35.1854236641678 / t1 / t51 * t35 + 825.342692284653 * t60 * t62 - 0.00350874814814815 * t5 * t66;
        let tv3rho30 = -3.72888 * t42 * t1 - 1.24296 * t21 * t18 + 0.276213333333333 * t8 * t39 - 0.93222 * t12 * t69;
        let t80 = t28 * t28;
        let t94 = t34 * t34;
        let tv4rho40 = -4.97184 * t69 * t1 - 2.48592 * t42 * t18 + 1.10485333333333 * t21 * t39 - 0.460355555555556 * t8 * t66 - 0.93222 * t12 * (-0.987654320987654 / t80 * t14 + 130.316383941362 / t1 / t80 * t35 - 5502.28461523102 / t17 / t80 * t62 + 87120.0968884812 / t80 / rho[ip] / t94 + 0.00935666172839506 * t5 * t60);
        zk[ip] += tzk0;
        vrho[ip] += tvrho0;
        v2rho2[ip] += tv2rho20;
        v3rho3[ip] += tv3rho30;
        v4rho4[ip] += tv4rho40;
    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip * 2] + rho[ip * 2 + 1];
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.556270992503 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        zk[ip] += tzk0;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip * 2] + rho[ip * 2 + 1];
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.556270992503 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.333333333333333 / t1 * t15 - 0.00315787333333333 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        let tvrho1 = tvrho0;
        zk[ip] += tzk0;
        vrho[ip * 2 + 0] += tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip * 2] + rho[ip * 2 + 1];
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.556270992503 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.333333333333333 / t1 * t15 - 0.00315787333333333 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        let tvrho1 = tvrho0;
        let t29 = t1 * t1;
        let t35 = t5 * t5;
        let t36 = 1.0 / t35;
        let t40 = 1.0 / t18 / t1;
        let t43 = -0.222222222222222 / t29 * t15 + 11.7284745547226 / t2 / t29 * t36 + 0.00210524888888889 * t6 * t40;
        let tv2rho20 = -2.48592 * t22 * t2 - 0.41432 * t9 * t19 - 0.93222 * t13 * t43;
        let tv2rho21 = tv2rho20;
        let tv2rho22 = tv2rho21;
        zk[ip] += tzk0;
        vrho[ip * 2 + 0] += tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        v2rho2[ip * 3 + 0] += tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip * 2] + rho[ip * 2 + 1];
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.556270992503 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.333333333333333 / t1 * t15 - 0.00315787333333333 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        let tvrho1 = tvrho0;
        let t29 = t1 * t1;
        let t35 = t5 * t5;
        let t36 = 1.0 / t35;
        let t40 = 1.0 / t18 / t1;
        let t43 = -0.222222222222222 / t29 * t15 + 11.7284745547226 / t2 / t29 * t36 + 0.00210524888888889 * t6 * t40;
        let tv2rho20 = -2.48592 * t22 * t2 - 0.41432 * t9 * t19 - 0.93222 * t13 * t43;
        let tv2rho21 = tv2rho20;
        let tv2rho22 = tv2rho21;
        let t52 = t29 * t1;
        let t61 = 1.0 / t18 / t52;
        let t63 = 1.0 / t35 / t5;
        let t67 = 1.0 / t18 / t29;
        let t70 = 0.37037037037037 / t52 * t15 - 35.1854236641678 / t2 / t52 * t36 + 825.342692284653 * t61 * t63 - 0.00350874814814815 * t6 * t67;
        let tv3rho30 = -3.72888 * t43 * t2 - 1.24296 * t22 * t19 + 0.276213333333333 * t9 * t40 - 0.93222 * t13 * t70;
        let tv3rho31 = tv3rho30;
        let tv3rho32 = tv3rho31;
        let tv3rho33 = tv3rho32;
        zk[ip] += tzk0;
        vrho[ip * 2 + 0] += tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        v2rho2[ip * 3 + 0] += tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        v3rho3[ip * 4 + 0] += tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = rho[ip * 2] + rho[ip * 2 + 1];
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.556270992503 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.333333333333333 / t1 * t15 - 0.00315787333333333 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        let tvrho1 = tvrho0;
        let t29 = t1 * t1;
        let t35 = t5 * t5;
        let t36 = 1.0 / t35;
        let t40 = 1.0 / t18 / t1;
        let t43 = -0.222222222222222 / t29 * t15 + 11.7284745547226 / t2 / t29 * t36 + 0.00210524888888889 * t6 * t40;
        let tv2rho20 = -2.48592 * t22 * t2 - 0.41432 * t9 * t19 - 0.93222 * t13 * t43;
        let tv2rho21 = tv2rho20;
        let tv2rho22 = tv2rho21;
        let t52 = t29 * t1;
        let t61 = 1.0 / t18 / t52;
        let t63 = 1.0 / t35 / t5;
        let t67 = 1.0 / t18 / t29;
        let t70 = 0.37037037037037 / t52 * t15 - 35.1854236641678 / t2 / t52 * t36 + 825.342692284653 * t61 * t63 - 0.00350874814814815 * t6 * t67;
        let tv3rho30 = -3.72888 * t43 * t2 - 1.24296 * t22 * t19 + 0.276213333333333 * t9 * t40 - 0.93222 * t13 * t70;
        let tv3rho31 = tv3rho30;
        let tv3rho32 = tv3rho31;
        let tv3rho33 = tv3rho32;
        let t81 = t29 * t29;
        let t95 = t35 * t35;
        let tv4rho40 = -4.97184 * t70 * t2 - 2.48592 * t43 * t19 + 1.10485333333333 * t22 * t40 - 0.460355555555556 * t9 * t67 - 0.93222 * t13 * (-0.987654320987654 / t81 * t15 + 130.316383941362 / t2 / t81 * t36 - 5502.28461523102 / t18 / t81 * t63 + 87120.0968884812 / t81 / t1 / t95 + 0.00935666172839506 * t6 * t61);
        let tv4rho41 = tv4rho40;
        let tv4rho42 = tv4rho41;
        let tv4rho43 = tv4rho42;
        let tv4rho44 = tv4rho43;
        zk[ip] += tzk0;
        vrho[ip * 2 + 0] += tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        v2rho2[ip * 3 + 0] += tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        v3rho3[ip * 4 + 0] += tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
        v4rho4[ip * 5 + 0] += tv4rho40;
        v4rho4[ip * 5 + 1] += tv4rho41;
        v4rho4[ip * 5 + 2] += tv4rho42;
        v4rho4[ip * 5 + 3] += tv4rho43;
        v4rho4[ip * 5 + 4] += tv4rho44;
    }
}
