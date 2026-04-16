//! LDA_C_WIGNER kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_wigner.c`.
//! Translation preserves exact maple2c variable names and operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT3, M_CBRT4, M_PI};
use crate::math::powers::{pow_1_3};

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_WIGNER exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t12 = param_b + t4 * t6 * t8 / 4.0;
        let tzk0 = param_a / t12;
        zk[ip] += tzk0;
    }
}

/// LDA_C_WIGNER vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t12 = param_b + t4 * t6 * t8 / 4.0;
        let tzk0 = param_a / t12;
        zk[ip] += tzk0;
        let t15 = t12 * t12;
        let t16 = 1.0 / t15;
        let tvrho0 = tzk0 + t8 * param_a * t16 * t4 * t6 / 12.0;
        vrho[ip] += tvrho0;
    }
}

/// LDA_C_WIGNER fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t12 = param_b + t4 * t6 * t8 / 4.0;
        let tzk0 = param_a / t12;
        zk[ip] += tzk0;
        let t15 = t12 * t12;
        let t16 = 1.0 / t15;
        let tvrho0 = tzk0 + t8 * param_a * t16 * t4 * t6 / 12.0;
        vrho[ip] += tvrho0;
        let t22 = param_a * t16 * t1;
        let t23 = t3 * t6;
        let t28 = t7 * t7;
        let t33 = 1.0 / t15 / t12;
        let t35 = t1 * t1;
        let t36 = t3 * t3;
        let tv2rho20 = t22 * t23 / t7 / rho[ip] / 18.0 + 1.0 / t28 / rho[ip] * param_a * t33 * t35 * t36 * t5 / 18.0;
        v2rho2[ip] += tv2rho20;
    }
}

/// LDA_C_WIGNER kxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t12 = param_b + t4 * t6 * t8 / 4.0;
        let tzk0 = param_a / t12;
        zk[ip] += tzk0;
        let t15 = t12 * t12;
        let t16 = 1.0 / t15;
        let tvrho0 = tzk0 + t8 * param_a * t16 * t4 * t6 / 12.0;
        vrho[ip] += tvrho0;
        let t22 = param_a * t16 * t1;
        let t23 = t3 * t6;
        let t28 = t7 * t7;
        let t33 = 1.0 / t15 / t12;
        let t35 = t1 * t1;
        let t36 = t3 * t3;
        let tv2rho20 = t22 * t23 / t7 / rho[ip] / 18.0 + 1.0 / t28 / rho[ip] * param_a * t33 * t35 * t36 * t5 / 18.0;
        v2rho2[ip] += tv2rho20;
        let t42 = param_a * t33 * t35;
        let t43 = t36 * t5;
        let t44 = rho[ip] * rho[ip];
        let t55 = t44 * rho[ip];
        let t58 = t15 * t15;
        let t59 = 1.0 / t58;
        let tv3rho30 = -t42 * t43 / t28 / t44 / 18.0 - 2.0 / 27.0 * t22 * t23 / t7 / t44 + 1.0 / t55 * param_a * t59 * t2 / 6.0;
        v3rho3[ip] += tv3rho30;
    }
}

/// LDA_C_WIGNER lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t12 = param_b + t4 * t6 * t8 / 4.0;
        let tzk0 = param_a / t12;
        zk[ip] += tzk0;
        let t15 = t12 * t12;
        let t16 = 1.0 / t15;
        let tvrho0 = tzk0 + t8 * param_a * t16 * t4 * t6 / 12.0;
        vrho[ip] += tvrho0;
        let t22 = param_a * t16 * t1;
        let t23 = t3 * t6;
        let t28 = t7 * t7;
        let t33 = 1.0 / t15 / t12;
        let t35 = t1 * t1;
        let t36 = t3 * t3;
        let tv2rho20 = t22 * t23 / t7 / rho[ip] / 18.0 + 1.0 / t28 / rho[ip] * param_a * t33 * t35 * t36 * t5 / 18.0;
        v2rho2[ip] += tv2rho20;
        let t42 = param_a * t33 * t35;
        let t43 = t36 * t5;
        let t44 = rho[ip] * rho[ip];
        let t55 = t44 * rho[ip];
        let t58 = t15 * t15;
        let t59 = 1.0 / t58;
        let tv3rho30 = -t42 * t43 / t28 / t44 / 18.0 - 2.0 / 27.0 * t22 * t23 / t7 / t44 + 1.0 / t55 * param_a * t59 * t2 / 6.0;
        v3rho3[ip] += tv3rho30;
        let t64 = t44 * t44;
        let tv4rho40 = -2.0 / 3.0 * param_a * t59 * t2 / t64 + 8.0 / 81.0 * t42 * t43 / t28 / t55 + 14.0 / 81.0 * t22 * t23 / t7 / t55 + 1.0 / t7 / t64 * param_a / t58 / t12 * t2 * t1 * t23 / 18.0;
        v4rho4[ip] += tv4rho40;
    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_WIGNER exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = t7 * param_a;
        let t9 = M_CBRT3;
        let t10 = 1.0 / M_PI;
        let t11 = pow_1_3(t10);
        let t12 = t9 * t11;
        let t13 = M_CBRT4;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t3);
        let t16 = 1.0 / t15;
        let t20 = param_b + t12 * t14 * t16 / 4.0;
        let t21 = 1.0 / t20;
        let tzk0 = t8 * t21;
        zk[ip] += tzk0;
    }
}

/// LDA_C_WIGNER vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = t7 * param_a;
        let t9 = M_CBRT3;
        let t10 = 1.0 / M_PI;
        let t11 = pow_1_3(t10);
        let t12 = t9 * t11;
        let t13 = M_CBRT4;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t3);
        let t16 = 1.0 / t15;
        let t20 = param_b + t12 * t14 * t16 / 4.0;
        let t21 = 1.0 / t20;
        let tzk0 = t8 * t21;
        zk[ip] += tzk0;
        let t22 = t1 * t5;
        let t23 = t4 * t3;
        let t24 = 1.0 / t23;
        let t25 = t2 * t24;
        let t27 = -2.0 * t22 + 2.0 * t25;
        let t29 = param_a * t21;
        let t33 = t20 * t20;
        let t34 = 1.0 / t33;
        let t36 = t11 * t14;
        let t37 = t34 * t9 * t36;
        let t39 = t16 * t7 * param_a * t37 / 12.0;
        let tvrho0 = t3 * t27 * t29 + t39 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t41 = 2.0 * t22 + 2.0 * t25;
        let tvrho1 = t3 * t41 * t29 + t39 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}

/// LDA_C_WIGNER fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = t7 * param_a;
        let t9 = M_CBRT3;
        let t10 = 1.0 / M_PI;
        let t11 = pow_1_3(t10);
        let t12 = t9 * t11;
        let t13 = M_CBRT4;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t3);
        let t16 = 1.0 / t15;
        let t20 = param_b + t12 * t14 * t16 / 4.0;
        let t21 = 1.0 / t20;
        let tzk0 = t8 * t21;
        zk[ip] += tzk0;
        let t22 = t1 * t5;
        let t23 = t4 * t3;
        let t24 = 1.0 / t23;
        let t25 = t2 * t24;
        let t27 = -2.0 * t22 + 2.0 * t25;
        let t29 = param_a * t21;
        let t33 = t20 * t20;
        let t34 = 1.0 / t33;
        let t36 = t11 * t14;
        let t37 = t34 * t9 * t36;
        let t39 = t16 * t7 * param_a * t37 / 12.0;
        let tvrho0 = t3 * t27 * t29 + t39 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t41 = 2.0 * t22 + 2.0 * t25;
        let tvrho1 = t3 * t41 * t29 + t39 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t44 = t27 * param_a;
        let t45 = t44 * t21;
        let t47 = t8 * t34;
        let t51 = t12 * t14 / t15 / t3;
        let t53 = t47 * t51 / 18.0;
        let t54 = 2.0 * t5;
        let t56 = 8.0 * t1 * t24;
        let t57 = t4 * t4;
        let t58 = 1.0 / t57;
        let t60 = 6.0 * t2 * t58;
        let t61 = -t54 + t56 - t60;
        let t66 = t16 * t27 * param_a * t37;
        let t68 = t15 * t15;
        let t70 = 1.0 / t68 / t3;
        let t74 = 1.0 / t33 / t20;
        let t75 = t9 * t9;
        let t77 = t11 * t11;
        let t79 = t74 * t75 * t77 * t13;
        let t81 = t70 * t7 * param_a * t79 / 18.0;
        let tv2rho20 = 2.0 * t45 + t53 + t3 * t61 * t29 + t66 / 6.0 + t81;
        v2rho2[ip * 3] += tv2rho20;
        let t82 = t41 * param_a;
        let t83 = t82 * t21;
        let t84 = t54 - t60;
        let t89 = t16 * t41 * param_a * t37;
        let tv2rho21 = t45 + t53 + t83 + t3 * t84 * t29 + t89 / 12.0 + t66 / 12.0 + t81;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t93 = -t54 - t56 - t60;
        let tv2rho22 = 2.0 * t83 + t53 + t3 * t93 * t29 + t89 / 6.0 + t81;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

/// LDA_C_WIGNER kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = t7 * param_a;
        let t9 = M_CBRT3;
        let t10 = 1.0 / M_PI;
        let t11 = pow_1_3(t10);
        let t12 = t9 * t11;
        let t13 = M_CBRT4;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t3);
        let t16 = 1.0 / t15;
        let t20 = param_b + t12 * t14 * t16 / 4.0;
        let t21 = 1.0 / t20;
        let tzk0 = t8 * t21;
        zk[ip] += tzk0;
        let t22 = t1 * t5;
        let t23 = t4 * t3;
        let t24 = 1.0 / t23;
        let t25 = t2 * t24;
        let t27 = -2.0 * t22 + 2.0 * t25;
        let t29 = param_a * t21;
        let t33 = t20 * t20;
        let t34 = 1.0 / t33;
        let t36 = t11 * t14;
        let t37 = t34 * t9 * t36;
        let t39 = t16 * t7 * param_a * t37 / 12.0;
        let tvrho0 = t3 * t27 * t29 + t39 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t41 = 2.0 * t22 + 2.0 * t25;
        let tvrho1 = t3 * t41 * t29 + t39 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t44 = t27 * param_a;
        let t45 = t44 * t21;
        let t47 = t8 * t34;
        let t51 = t12 * t14 / t15 / t3;
        let t53 = t47 * t51 / 18.0;
        let t54 = 2.0 * t5;
        let t56 = 8.0 * t1 * t24;
        let t57 = t4 * t4;
        let t58 = 1.0 / t57;
        let t60 = 6.0 * t2 * t58;
        let t61 = -t54 + t56 - t60;
        let t66 = t16 * t27 * param_a * t37;
        let t68 = t15 * t15;
        let t70 = 1.0 / t68 / t3;
        let t74 = 1.0 / t33 / t20;
        let t75 = t9 * t9;
        let t77 = t11 * t11;
        let t79 = t74 * t75 * t77 * t13;
        let t81 = t70 * t7 * param_a * t79 / 18.0;
        let tv2rho20 = 2.0 * t45 + t53 + t3 * t61 * t29 + t66 / 6.0 + t81;
        v2rho2[ip * 3] += tv2rho20;
        let t82 = t41 * param_a;
        let t83 = t82 * t21;
        let t84 = t54 - t60;
        let t89 = t16 * t41 * param_a * t37;
        let tv2rho21 = t45 + t53 + t83 + t3 * t84 * t29 + t89 / 12.0 + t66 / 12.0 + t81;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t93 = -t54 - t56 - t60;
        let tv2rho22 = 2.0 * t83 + t53 + t3 * t93 * t29 + t89 / 6.0 + t81;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t97 = t61 * param_a;
        let t98 = t97 * t21;
        let t100 = t44 * t34;
        let t101 = t100 * t51;
        let t103 = t8 * t74;
        let t104 = t75 * t77;
        let t108 = t104 * t13 / t68 / t4;
        let t110 = t103 * t108 / 18.0;
        let t114 = t12 * t14 / t15 / t4;
        let t116 = 2.0 / 27.0 * t47 * t114;
        let t117 = 12.0 * t24;
        let t118 = t1 * t58;
        let t119 = 36.0 * t118;
        let t121 = 1.0 / t57 / t3;
        let t123 = 24.0 * t2 * t121;
        let t124 = t117 - t119 + t123;
        let t129 = t16 * t61 * param_a * t37;
        let t133 = t70 * t27 * param_a * t79;
        let t136 = t33 * t33;
        let t137 = 1.0 / t136;
        let t139 = param_a * t137 * t10;
        let t141 = t24 * t7 * t139 / 6.0;
        let tv3rho30 = 3.0 * t98 + t101 / 6.0 - t110 - t116 + t3 * t124 * t29 + t129 / 4.0 + t133 / 6.0 + t141;
        v3rho3[ip * 4] += tv3rho30;
        let t143 = t84 * param_a;
        let t145 = 2.0 * t143 * t21;
        let t146 = t82 * t34;
        let t147 = t146 * t51;
        let t149 = 4.0 * t24;
        let t150 = 12.0 * t118;
        let t151 = -t149 - t150 + t123;
        let t157 = t16 * t84 * param_a * t37 / 6.0;
        let t160 = t70 * t41 * param_a * t79;
        let tv3rho31 = t98 + t101 / 9.0 - t110 - t116 + t145 + t147 / 18.0 + t3 * t151 * t29 + t157 + t160 / 18.0 + t129 / 12.0 + t133 / 9.0 + t141;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t166 = t93 * param_a;
        let t167 = t166 * t21;
        let t168 = -t149 + t150 + t123;
        let t173 = t16 * t93 * param_a * t37;
        let tv3rho32 = t145 + t147 / 9.0 + t101 / 18.0 - t110 - t116 + t167 + t3 * t168 * t29 + t173 / 12.0 + t157 + t160 / 9.0 + t133 / 18.0 + t141;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t179 = t117 + t119 + t123;
        let tv3rho33 = 3.0 * t167 + t147 / 6.0 - t110 - t116 + t3 * t179 * t29 + t173 / 4.0 + t160 / 6.0 + t141;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

/// LDA_C_WIGNER lxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_wigner_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = t7 * param_a;
        let t9 = M_CBRT3;
        let t10 = 1.0 / M_PI;
        let t11 = pow_1_3(t10);
        let t12 = t9 * t11;
        let t13 = M_CBRT4;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t3);
        let t16 = 1.0 / t15;
        let t20 = param_b + t12 * t14 * t16 / 4.0;
        let t21 = 1.0 / t20;
        let tzk0 = t8 * t21;
        zk[ip] += tzk0;
        let t22 = t1 * t5;
        let t23 = t4 * t3;
        let t24 = 1.0 / t23;
        let t25 = t2 * t24;
        let t27 = -2.0 * t22 + 2.0 * t25;
        let t29 = param_a * t21;
        let t33 = t20 * t20;
        let t34 = 1.0 / t33;
        let t36 = t11 * t14;
        let t37 = t34 * t9 * t36;
        let t39 = t16 * t7 * param_a * t37 / 12.0;
        let tvrho0 = t3 * t27 * t29 + t39 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t41 = 2.0 * t22 + 2.0 * t25;
        let tvrho1 = t3 * t41 * t29 + t39 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t44 = t27 * param_a;
        let t45 = t44 * t21;
        let t47 = t8 * t34;
        let t51 = t12 * t14 / t15 / t3;
        let t53 = t47 * t51 / 18.0;
        let t54 = 2.0 * t5;
        let t56 = 8.0 * t1 * t24;
        let t57 = t4 * t4;
        let t58 = 1.0 / t57;
        let t60 = 6.0 * t2 * t58;
        let t61 = -t54 + t56 - t60;
        let t66 = t16 * t27 * param_a * t37;
        let t68 = t15 * t15;
        let t70 = 1.0 / t68 / t3;
        let t74 = 1.0 / t33 / t20;
        let t75 = t9 * t9;
        let t77 = t11 * t11;
        let t79 = t74 * t75 * t77 * t13;
        let t81 = t70 * t7 * param_a * t79 / 18.0;
        let tv2rho20 = 2.0 * t45 + t53 + t3 * t61 * t29 + t66 / 6.0 + t81;
        v2rho2[ip * 3] += tv2rho20;
        let t82 = t41 * param_a;
        let t83 = t82 * t21;
        let t84 = t54 - t60;
        let t89 = t16 * t41 * param_a * t37;
        let tv2rho21 = t45 + t53 + t83 + t3 * t84 * t29 + t89 / 12.0 + t66 / 12.0 + t81;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t93 = -t54 - t56 - t60;
        let tv2rho22 = 2.0 * t83 + t53 + t3 * t93 * t29 + t89 / 6.0 + t81;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t97 = t61 * param_a;
        let t98 = t97 * t21;
        let t100 = t44 * t34;
        let t101 = t100 * t51;
        let t103 = t8 * t74;
        let t104 = t75 * t77;
        let t108 = t104 * t13 / t68 / t4;
        let t110 = t103 * t108 / 18.0;
        let t114 = t12 * t14 / t15 / t4;
        let t116 = 2.0 / 27.0 * t47 * t114;
        let t117 = 12.0 * t24;
        let t118 = t1 * t58;
        let t119 = 36.0 * t118;
        let t121 = 1.0 / t57 / t3;
        let t123 = 24.0 * t2 * t121;
        let t124 = t117 - t119 + t123;
        let t129 = t16 * t61 * param_a * t37;
        let t133 = t70 * t27 * param_a * t79;
        let t136 = t33 * t33;
        let t137 = 1.0 / t136;
        let t139 = param_a * t137 * t10;
        let t141 = t24 * t7 * t139 / 6.0;
        let tv3rho30 = 3.0 * t98 + t101 / 6.0 - t110 - t116 + t3 * t124 * t29 + t129 / 4.0 + t133 / 6.0 + t141;
        v3rho3[ip * 4] += tv3rho30;
        let t143 = t84 * param_a;
        let t145 = 2.0 * t143 * t21;
        let t146 = t82 * t34;
        let t147 = t146 * t51;
        let t149 = 4.0 * t24;
        let t150 = 12.0 * t118;
        let t151 = -t149 - t150 + t123;
        let t157 = t16 * t84 * param_a * t37 / 6.0;
        let t160 = t70 * t41 * param_a * t79;
        let tv3rho31 = t98 + t101 / 9.0 - t110 - t116 + t145 + t147 / 18.0 + t3 * t151 * t29 + t157 + t160 / 18.0 + t129 / 12.0 + t133 / 9.0 + t141;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t166 = t93 * param_a;
        let t167 = t166 * t21;
        let t168 = -t149 + t150 + t123;
        let t173 = t16 * t93 * param_a * t37;
        let tv3rho32 = t145 + t147 / 9.0 + t101 / 18.0 - t110 - t116 + t167 + t3 * t168 * t29 + t173 / 12.0 + t157 + t160 / 9.0 + t133 / 18.0 + t141;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t179 = t117 + t119 + t123;
        let tv3rho33 = 3.0 * t167 + t147 / 6.0 - t110 - t116 + t3 * t179 * t29 + t173 / 4.0 + t160 / 6.0 + t141;
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t185 = t124 * param_a * t21;
        let t188 = t97 * t34 * t51;
        let t191 = t44 * t74 * t108;
        let t193 = t100 * t114;
        let t198 = 2.0 / 3.0 * t8 * t137 * t10 * t58;
        let t204 = 8.0 / 81.0 * t103 * t104 * t13 / t68 / t23;
        let t210 = 14.0 / 81.0 * t47 * t12 * t14 / t15 / t23;
        let t211 = 72.0 * t58;
        let t212 = t1 * t121;
        let t213 = 192.0 * t212;
        let t217 = 120.0 * t2 / t57 / t4;
        let t223 = t16 * t124 * param_a * t37;
        let t227 = t70 * t61 * param_a * t79;
        let t230 = t24 * t27 * t139;
        let t242 = 1.0 / t15 / t57 * t7 * param_a / t136 / t20 * t10 * t9 * t36 / 18.0;
        let tv4rho40 = 4.0 * t185 + t188 / 3.0 - 2.0 / 9.0 * t191 - 8.0 / 27.0 * t193 - t198 + t204 + t210 + t3 * (-t211 + t213 - t217) * t29 + t223 / 3.0 + t227 / 3.0 + 2.0 / 3.0 * t230 + t242;
        v4rho4[ip * 5] += tv4rho40;
        let t245 = t24 * t41 * t139;
        let t247 = 96.0 * t212;
        let t252 = t146 * t114;
        let t255 = t151 * param_a * t21;
        let t262 = t143 * t34 * t51;
        let t263 = t262 / 6.0;
        let t265 = t82 * t74 * t108;
        let t269 = t16 * t151 * param_a * t37;
        let t273 = t70 * t84 * param_a * t79;
        let t274 = t273 / 6.0;
        let tv4rho41 = -t198 + t230 / 2.0 + t245 / 6.0 + t3 * (t247 - t217) * t29 - 2.0 / 9.0 * t193 + t204 + t210 + t242 - 2.0 / 27.0 * t252 + t185 + 3.0 * t255 + t188 / 6.0 - t191 / 6.0 + t223 / 12.0 + t227 / 6.0 + t263 - t265 / 18.0 + t269 / 4.0 + t274;
        v4rho4[ip * 5 + 1] += tv4rho41;
        let t286 = t168 * param_a * t21;
        let t296 = t166 * t34 * t51;
        let t300 = t16 * t168 * param_a * t37;
        let t304 = t70 * t93 * param_a * t79;
        let t306 = 2.0 * t286 + t188 / 18.0 - t191 / 9.0 + t227 / 18.0 + 2.0 / 9.0 * t262 - t265 / 9.0 + t269 / 6.0 + 2.0 / 9.0 * t273 + t296 / 18.0 + t300 / 6.0 + t304 / 18.0;
        let tv4rho42 = -t198 + t230 / 3.0 + t245 / 3.0 + t3 * (24.0 * t58 - t217) * t29 - 4.0 / 27.0 * t193 + t204 + t210 + t242 - 4.0 / 27.0 * t252 + 2.0 * t255 + t306;
        v4rho4[ip * 5 + 2] += tv4rho42;
        let t314 = t179 * param_a * t21;
        let t320 = t16 * t179 * param_a * t37;
        let tv4rho43 = 3.0 * t286 + t296 / 6.0 + t263 - t265 / 6.0 - 2.0 / 9.0 * t252 - t191 / 18.0 - t198 + t204 - 2.0 / 27.0 * t193 + t210 + t314 + t3 * (-t247 - t217) * t29 + t320 / 12.0 + t300 / 4.0 + t304 / 6.0 + t274 + t245 / 2.0 + t230 / 6.0 + t242;
        v4rho4[ip * 5 + 3] += tv4rho43;
        let tv4rho44 = 4.0 * t314 + t296 / 3.0 - 2.0 / 9.0 * t265 - 8.0 / 27.0 * t252 - t198 + t204 + t210 + t3 * (-t211 - t213 - t217) * t29 + t320 / 3.0 + t304 / 3.0 + 2.0 / 3.0 * t245 + t242;
        v4rho4[ip * 5 + 4] += tv4rho44;
    }
}
