//! LDA_K_GDS08_WORKER kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_gds08_worker.c`.
//! Translation preserves exact maple2c variable names and operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::piecewise::{piecewise3, piecewise5};

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_K_GDS08_WORKER exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = 1.0 <= zeta_threshold;
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5(t3, t4, t3, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t9 = f64::ln(t7 * rho[ip]);
        let t11 = t9 * t9;
        let t16 = piecewise3(t2, 0.0, t7 * (param_C * t11 + param_B * t9 + param_A) / 2.0);
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
    }
}

/// LDA_K_GDS08_WORKER vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = 1.0 <= zeta_threshold;
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5(t3, t4, t3, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t9 = f64::ln(t7 * rho[ip]);
        let t11 = t9 * t9;
        let t16 = piecewise3(t2, 0.0, t7 * (param_C * t11 + param_B * t9 + param_A) / 2.0);
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
        let t17 = 1.0 / rho[ip];
        let t19 = param_C * t9;
        let t25 = piecewise3(t2, 0.0, t7 * (2.0 * t19 * t17 + param_B * t17) / 2.0);
        let tvrho0 = 2.0 * rho[ip] * t25 + 2.0 * t16;
        vrho[ip] += tvrho0;
    }
}

/// LDA_K_GDS08_WORKER fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = 1.0 <= zeta_threshold;
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5(t3, t4, t3, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t9 = f64::ln(t7 * rho[ip]);
        let t11 = t9 * t9;
        let t16 = piecewise3(t2, 0.0, t7 * (param_C * t11 + param_B * t9 + param_A) / 2.0);
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
        let t17 = 1.0 / rho[ip];
        let t19 = param_C * t9;
        let t25 = piecewise3(t2, 0.0, t7 * (2.0 * t19 * t17 + param_B * t17) / 2.0);
        let tvrho0 = 2.0 * rho[ip] * t25 + 2.0 * t16;
        vrho[ip] += tvrho0;
        let t29 = rho[ip] * rho[ip];
        let t30 = 1.0 / t29;
        let t39 = piecewise3(t2, 0.0, t7 * (-2.0 * t19 * t30 - param_B * t30 + 2.0 * param_C * t30) / 2.0);
        let tv2rho20 = 2.0 * rho[ip] * t39 + 4.0 * t25;
        v2rho2[ip] += tv2rho20;
    }
}

/// LDA_K_GDS08_WORKER kxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = 1.0 <= zeta_threshold;
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5(t3, t4, t3, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t9 = f64::ln(t7 * rho[ip]);
        let t11 = t9 * t9;
        let t16 = piecewise3(t2, 0.0, t7 * (param_C * t11 + param_B * t9 + param_A) / 2.0);
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
        let t17 = 1.0 / rho[ip];
        let t19 = param_C * t9;
        let t25 = piecewise3(t2, 0.0, t7 * (2.0 * t19 * t17 + param_B * t17) / 2.0);
        let tvrho0 = 2.0 * rho[ip] * t25 + 2.0 * t16;
        vrho[ip] += tvrho0;
        let t29 = rho[ip] * rho[ip];
        let t30 = 1.0 / t29;
        let t39 = piecewise3(t2, 0.0, t7 * (-2.0 * t19 * t30 - param_B * t30 + 2.0 * param_C * t30) / 2.0);
        let tv2rho20 = 2.0 * rho[ip] * t39 + 4.0 * t25;
        v2rho2[ip] += tv2rho20;
        let t44 = 1.0 / t29 / rho[ip];
        let t54 = piecewise3(t2, 0.0, t7 * (4.0 * t19 * t44 + 2.0 * param_B * t44 - 6.0 * param_C * t44) / 2.0);
        let tv3rho30 = 2.0 * rho[ip] * t54 + 6.0 * t39;
        v3rho3[ip] += tv3rho30;
    }
}

/// LDA_K_GDS08_WORKER lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = 1.0 <= zeta_threshold;
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5(t3, t4, t3, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t9 = f64::ln(t7 * rho[ip]);
        let t11 = t9 * t9;
        let t16 = piecewise3(t2, 0.0, t7 * (param_C * t11 + param_B * t9 + param_A) / 2.0);
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
        let t17 = 1.0 / rho[ip];
        let t19 = param_C * t9;
        let t25 = piecewise3(t2, 0.0, t7 * (2.0 * t19 * t17 + param_B * t17) / 2.0);
        let tvrho0 = 2.0 * rho[ip] * t25 + 2.0 * t16;
        vrho[ip] += tvrho0;
        let t29 = rho[ip] * rho[ip];
        let t30 = 1.0 / t29;
        let t39 = piecewise3(t2, 0.0, t7 * (-2.0 * t19 * t30 - param_B * t30 + 2.0 * param_C * t30) / 2.0);
        let tv2rho20 = 2.0 * rho[ip] * t39 + 4.0 * t25;
        v2rho2[ip] += tv2rho20;
        let t44 = 1.0 / t29 / rho[ip];
        let t54 = piecewise3(t2, 0.0, t7 * (4.0 * t19 * t44 + 2.0 * param_B * t44 - 6.0 * param_C * t44) / 2.0);
        let tv3rho30 = 2.0 * rho[ip] * t54 + 6.0 * t39;
        v3rho3[ip] += tv3rho30;
        let t58 = t29 * t29;
        let t59 = 1.0 / t58;
        let t69 = piecewise3(t2, 0.0, t7 * (-12.0 * t19 * t59 - 6.0 * param_B * t59 + 22.0 * param_C * t59) / 2.0);
        let tv4rho40 = 2.0 * rho[ip] * t69 + 8.0 * t54;
        v4rho4[ip] += tv4rho40;
    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_K_GDS08_WORKER exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = 1.0 - t5 <= zeta_threshold;
        let t11 = -t8;
        let t12 = piecewise5(t7, t8, t10, t11, t5);
        let t13 = 1.0 + t12;
        let t16 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t19 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t20 = piecewise5(t16, t8, t19, t11, t5);
        let t21 = 1.0 + t20;
        let t23 = f64::ln(t21 * t3);
        let t25 = t23 * t23;
        let t27 = t23 * param_B + t25 * param_C + param_A;
        let t30 = piecewise3(t1, 0.0, t13 * t27 / 2.0);
        let t31 = rho1 <= dens_threshold;
        let t32 = piecewise5(t10, t8, t7, t11, -t5);
        let t33 = 1.0 + t32;
        let t34 = -t2;
        let t36 = piecewise5(t19, t8, t16, t11, t34 * t4);
        let t37 = 1.0 + t36;
        let t39 = f64::ln(t37 * t3);
        let t41 = t39 * t39;
        let t43 = t39 * param_B + t41 * param_C + param_A;
        let t46 = piecewise3(t31, 0.0, t33 * t43 / 2.0);
        let tzk0 = t30 + t46;
        zk[ip] += tzk0;
    }
}

/// LDA_K_GDS08_WORKER vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = 1.0 - t5 <= zeta_threshold;
        let t11 = -t8;
        let t12 = piecewise5(t7, t8, t10, t11, t5);
        let t13 = 1.0 + t12;
        let t16 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t19 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t20 = piecewise5(t16, t8, t19, t11, t5);
        let t21 = 1.0 + t20;
        let t23 = f64::ln(t21 * t3);
        let t25 = t23 * t23;
        let t27 = t23 * param_B + t25 * param_C + param_A;
        let t30 = piecewise3(t1, 0.0, t13 * t27 / 2.0);
        let t31 = rho1 <= dens_threshold;
        let t32 = piecewise5(t10, t8, t7, t11, -t5);
        let t33 = 1.0 + t32;
        let t34 = -t2;
        let t36 = piecewise5(t19, t8, t16, t11, t34 * t4);
        let t37 = 1.0 + t36;
        let t39 = f64::ln(t37 * t3);
        let t41 = t39 * t39;
        let t43 = t39 * param_B + t41 * param_C + param_A;
        let t46 = piecewise3(t31, 0.0, t33 * t43 / 2.0);
        let tzk0 = t30 + t46;
        zk[ip] += tzk0;
        let t47 = t3 * t3;
        let t48 = 1.0 / t47;
        let t49 = t2 * t48;
        let t50 = t4 - t49;
        let t51 = piecewise5(t7, 0.0, t10, 0.0, t50);
        let t53 = piecewise5(t16, 0.0, t19, 0.0, t50);
        let t55 = t3 * t53 + t20 + 1.0;
        let t56 = param_B * t55;
        let t57 = 1.0 / t21;
        let t58 = t57 * t4;
        let t60 = param_C * t23;
        let t61 = t55 * t57;
        let t65 = 2.0 * t4 * t60 * t61 + t56 * t58;
        let t69 = piecewise3(t1, 0.0, t13 * t65 / 2.0 + t51 * t27 / 2.0);
        let t71 = piecewise5(t10, 0.0, t7, 0.0, -t50);
        let t73 = t34 * t48;
        let t75 = piecewise5(t19, 0.0, t16, 0.0, -t4 - t73);
        let t77 = t3 * t75 + t36 + 1.0;
        let t78 = param_B * t77;
        let t79 = 1.0 / t37;
        let t80 = t79 * t4;
        let t82 = param_C * t39;
        let t83 = t77 * t79;
        let t87 = 2.0 * t4 * t82 * t83 + t78 * t80;
        let t91 = piecewise3(t31, 0.0, t33 * t87 / 2.0 + t71 * t43 / 2.0);
        let tvrho0 = t30 + t46 + t3 * (t69 + t91);
        vrho[ip * 2] += tvrho0;
        let t94 = -t4 - t49;
        let t95 = piecewise5(t7, 0.0, t10, 0.0, t94);
        let t97 = piecewise5(t16, 0.0, t19, 0.0, t94);
        let t99 = t3 * t97 + t20 + 1.0;
        let t100 = param_B * t99;
        let t102 = t99 * t57;
        let t106 = 2.0 * t102 * t4 * t60 + t100 * t58;
        let t110 = piecewise3(t1, 0.0, t13 * t106 / 2.0 + t95 * t27 / 2.0);
        let t112 = piecewise5(t10, 0.0, t7, 0.0, -t94);
        let t115 = piecewise5(t19, 0.0, t16, 0.0, t4 - t73);
        let t117 = t115 * t3 + t36 + 1.0;
        let t118 = param_B * t117;
        let t120 = t117 * t79;
        let t124 = 2.0 * t120 * t4 * t82 + t118 * t80;
        let t128 = piecewise3(t31, 0.0, t112 * t43 / 2.0 + t33 * t124 / 2.0);
        let tvrho1 = t30 + t46 + t3 * (t110 + t128);
        vrho[ip * 2 + 1] += tvrho1;
    }
}

/// LDA_K_GDS08_WORKER fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = 1.0 - t5 <= zeta_threshold;
        let t11 = -t8;
        let t12 = piecewise5(t7, t8, t10, t11, t5);
        let t13 = 1.0 + t12;
        let t16 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t19 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t20 = piecewise5(t16, t8, t19, t11, t5);
        let t21 = 1.0 + t20;
        let t23 = f64::ln(t21 * t3);
        let t25 = t23 * t23;
        let t27 = t23 * param_B + t25 * param_C + param_A;
        let t30 = piecewise3(t1, 0.0, t13 * t27 / 2.0);
        let t31 = rho1 <= dens_threshold;
        let t32 = piecewise5(t10, t8, t7, t11, -t5);
        let t33 = 1.0 + t32;
        let t34 = -t2;
        let t36 = piecewise5(t19, t8, t16, t11, t34 * t4);
        let t37 = 1.0 + t36;
        let t39 = f64::ln(t37 * t3);
        let t41 = t39 * t39;
        let t43 = t39 * param_B + t41 * param_C + param_A;
        let t46 = piecewise3(t31, 0.0, t33 * t43 / 2.0);
        let tzk0 = t30 + t46;
        zk[ip] += tzk0;
        let t47 = t3 * t3;
        let t48 = 1.0 / t47;
        let t49 = t2 * t48;
        let t50 = t4 - t49;
        let t51 = piecewise5(t7, 0.0, t10, 0.0, t50);
        let t53 = piecewise5(t16, 0.0, t19, 0.0, t50);
        let t55 = t3 * t53 + t20 + 1.0;
        let t56 = param_B * t55;
        let t57 = 1.0 / t21;
        let t58 = t57 * t4;
        let t60 = param_C * t23;
        let t61 = t55 * t57;
        let t65 = 2.0 * t4 * t60 * t61 + t56 * t58;
        let t69 = piecewise3(t1, 0.0, t13 * t65 / 2.0 + t51 * t27 / 2.0);
        let t71 = piecewise5(t10, 0.0, t7, 0.0, -t50);
        let t73 = t34 * t48;
        let t75 = piecewise5(t19, 0.0, t16, 0.0, -t4 - t73);
        let t77 = t3 * t75 + t36 + 1.0;
        let t78 = param_B * t77;
        let t79 = 1.0 / t37;
        let t80 = t79 * t4;
        let t82 = param_C * t39;
        let t83 = t77 * t79;
        let t87 = 2.0 * t4 * t82 * t83 + t78 * t80;
        let t91 = piecewise3(t31, 0.0, t33 * t87 / 2.0 + t71 * t43 / 2.0);
        let tvrho0 = t30 + t46 + t3 * (t69 + t91);
        vrho[ip * 2] += tvrho0;
        let t94 = -t4 - t49;
        let t95 = piecewise5(t7, 0.0, t10, 0.0, t94);
        let t97 = piecewise5(t16, 0.0, t19, 0.0, t94);
        let t99 = t3 * t97 + t20 + 1.0;
        let t100 = param_B * t99;
        let t102 = t99 * t57;
        let t106 = 2.0 * t102 * t4 * t60 + t100 * t58;
        let t110 = piecewise3(t1, 0.0, t13 * t106 / 2.0 + t95 * t27 / 2.0);
        let t112 = piecewise5(t10, 0.0, t7, 0.0, -t94);
        let t115 = piecewise5(t19, 0.0, t16, 0.0, t4 - t73);
        let t117 = t115 * t3 + t36 + 1.0;
        let t118 = param_B * t117;
        let t120 = t117 * t79;
        let t124 = 2.0 * t120 * t4 * t82 + t118 * t80;
        let t128 = piecewise3(t31, 0.0, t112 * t43 / 2.0 + t33 * t124 / 2.0);
        let tvrho1 = t30 + t46 + t3 * (t110 + t128);
        vrho[ip * 2 + 1] += tvrho1;
        let t134 = 1.0 / t47 / t3;
        let t135 = t2 * t134;
        let t137 = -2.0 * t48 + 2.0 * t135;
        let t138 = piecewise5(t7, 0.0, t10, 0.0, t137);
        let t142 = piecewise5(t16, 0.0, t19, 0.0, t137);
        let t145 = t142 * t3 + 2.0 * t53;
        let t146 = param_B * t145;
        let t148 = t21 * t21;
        let t149 = 1.0 / t148;
        let t150 = t149 * t4;
        let t151 = t150 * t53;
        let t153 = t57 * t48;
        let t155 = t55 * t55;
        let t156 = param_C * t155;
        let t157 = t149 * t48;
        let t160 = t145 * t57;
        let t164 = t60 * t55;
        let t170 = 2.0 * t160 * t4 * t60 - 2.0 * t48 * t60 * t61 + t146 * t58 - 2.0 * t151 * t164 - t151 * t56 - t153 * t56 + 2.0 * t156 * t157;
        let t174 = piecewise3(t1, 0.0, t138 * t27 / 2.0 + t51 * t65 + t13 * t170 / 2.0);
        let t176 = piecewise5(t10, 0.0, t7, 0.0, -t137);
        let t180 = t34 * t134;
        let t183 = piecewise5(t19, 0.0, t16, 0.0, 2.0 * t48 + 2.0 * t180);
        let t186 = t183 * t3 + 2.0 * t75;
        let t187 = param_B * t186;
        let t189 = t37 * t37;
        let t190 = 1.0 / t189;
        let t191 = t190 * t4;
        let t192 = t191 * t75;
        let t194 = t79 * t48;
        let t196 = t77 * t77;
        let t197 = param_C * t196;
        let t198 = t190 * t48;
        let t201 = t186 * t79;
        let t205 = t82 * t77;
        let t211 = 2.0 * t201 * t4 * t82 - 2.0 * t48 * t82 * t83 + t187 * t80 - 2.0 * t192 * t205 - t192 * t78 - t194 * t78 + 2.0 * t197 * t198;
        let t215 = piecewise3(t31, 0.0, t176 * t43 / 2.0 + t71 * t87 + t33 * t211 / 2.0);
        let tv2rho20 = 2.0 * t69 + 2.0 * t91 + t3 * (t174 + t215);
        v2rho2[ip * 3] += tv2rho20;
        let t218 = 2.0 * t135;
        let t219 = piecewise5(t7, 0.0, t10, 0.0, t218);
        let t223 = piecewise5(t16, 0.0, t19, 0.0, t218);
        let t225 = t223 * t3 + t53 + t97;
        let t226 = param_B * t225;
        let t229 = t100 * t153;
        let t230 = param_C * t55;
        let t231 = t157 * t99;
        let t234 = t225 * t57;
        let t238 = t60 * t99;
        let t243 = 2.0 * t60 * t102 * t48;
        let t244 = 2.0 * t234 * t4 * t60 - t100 * t151 - 2.0 * t151 * t238 + t226 * t58 + 2.0 * t230 * t231 - t229 - t243;
        let t248 = piecewise3(t1, 0.0, t51 * t106 / 2.0 + t13 * t244 / 2.0 + t219 * t27 / 2.0 + t95 * t65 / 2.0);
        let t249 = piecewise5(t10, 0.0, t7, 0.0, -t218);
        let t254 = piecewise5(t19, 0.0, t16, 0.0, 2.0 * t180);
        let t256 = t254 * t3 + t115 + t75;
        let t257 = param_B * t256;
        let t260 = t118 * t194;
        let t261 = param_C * t77;
        let t262 = t198 * t117;
        let t265 = t256 * t79;
        let t269 = t82 * t117;
        let t274 = 2.0 * t82 * t120 * t48;
        let t275 = 2.0 * t265 * t4 * t82 - t118 * t192 - 2.0 * t192 * t269 + t257 * t80 + 2.0 * t261 * t262 - t260 - t274;
        let t279 = piecewise3(t31, 0.0, t112 * t87 / 2.0 + t71 * t124 / 2.0 + t249 * t43 / 2.0 + t33 * t275 / 2.0);
        let tv2rho21 = t69 + t91 + t110 + t128 + t3 * (t248 + t279);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t285 = 2.0 * t48 + 2.0 * t135;
        let t286 = piecewise5(t7, 0.0, t10, 0.0, t285);
        let t290 = piecewise5(t16, 0.0, t19, 0.0, t285);
        let t293 = t290 * t3 + 2.0 * t97;
        let t294 = param_B * t293;
        let t296 = t150 * t97;
        let t298 = t99 * t99;
        let t299 = param_C * t298;
        let t302 = t293 * t57;
        let t308 = 2.0 * t302 * t4 * t60 - t100 * t296 + 2.0 * t157 * t299 - 2.0 * t238 * t296 + t294 * t58 - t229 - t243;
        let t312 = piecewise3(t1, 0.0, t286 * t27 / 2.0 + t95 * t106 + t13 * t308 / 2.0);
        let t314 = piecewise5(t10, 0.0, t7, 0.0, -t285);
        let t320 = piecewise5(t19, 0.0, t16, 0.0, -2.0 * t48 + 2.0 * t180);
        let t323 = t3 * t320 + 2.0 * t115;
        let t324 = param_B * t323;
        let t326 = t191 * t115;
        let t328 = t117 * t117;
        let t329 = param_C * t328;
        let t332 = t323 * t79;
        let t338 = 2.0 * t332 * t4 * t82 - t118 * t326 + 2.0 * t198 * t329 - 2.0 * t269 * t326 + t324 * t80 - t260 - t274;
        let t342 = piecewise3(t31, 0.0, t314 * t43 / 2.0 + t112 * t124 + t33 * t338 / 2.0);
        let tv2rho22 = 2.0 * t110 + 2.0 * t128 + t3 * (t312 + t342);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

/// LDA_K_GDS08_WORKER kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = 1.0 - t5 <= zeta_threshold;
        let t11 = -t8;
        let t12 = piecewise5(t7, t8, t10, t11, t5);
        let t13 = 1.0 + t12;
        let t16 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t19 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t20 = piecewise5(t16, t8, t19, t11, t5);
        let t21 = 1.0 + t20;
        let t23 = f64::ln(t21 * t3);
        let t25 = t23 * t23;
        let t27 = t23 * param_B + t25 * param_C + param_A;
        let t30 = piecewise3(t1, 0.0, t13 * t27 / 2.0);
        let t31 = rho1 <= dens_threshold;
        let t32 = piecewise5(t10, t8, t7, t11, -t5);
        let t33 = 1.0 + t32;
        let t34 = -t2;
        let t36 = piecewise5(t19, t8, t16, t11, t34 * t4);
        let t37 = 1.0 + t36;
        let t39 = f64::ln(t37 * t3);
        let t41 = t39 * t39;
        let t43 = t39 * param_B + t41 * param_C + param_A;
        let t46 = piecewise3(t31, 0.0, t33 * t43 / 2.0);
        let tzk0 = t30 + t46;
        zk[ip] += tzk0;
        let t47 = t3 * t3;
        let t48 = 1.0 / t47;
        let t49 = t2 * t48;
        let t50 = t4 - t49;
        let t51 = piecewise5(t7, 0.0, t10, 0.0, t50);
        let t53 = piecewise5(t16, 0.0, t19, 0.0, t50);
        let t55 = t3 * t53 + t20 + 1.0;
        let t56 = param_B * t55;
        let t57 = 1.0 / t21;
        let t58 = t57 * t4;
        let t60 = param_C * t23;
        let t61 = t55 * t57;
        let t65 = 2.0 * t4 * t60 * t61 + t56 * t58;
        let t69 = piecewise3(t1, 0.0, t13 * t65 / 2.0 + t51 * t27 / 2.0);
        let t71 = piecewise5(t10, 0.0, t7, 0.0, -t50);
        let t73 = t34 * t48;
        let t75 = piecewise5(t19, 0.0, t16, 0.0, -t4 - t73);
        let t77 = t3 * t75 + t36 + 1.0;
        let t78 = param_B * t77;
        let t79 = 1.0 / t37;
        let t80 = t79 * t4;
        let t82 = param_C * t39;
        let t83 = t77 * t79;
        let t87 = 2.0 * t4 * t82 * t83 + t78 * t80;
        let t91 = piecewise3(t31, 0.0, t33 * t87 / 2.0 + t71 * t43 / 2.0);
        let tvrho0 = t30 + t46 + t3 * (t69 + t91);
        vrho[ip * 2] += tvrho0;
        let t94 = -t4 - t49;
        let t95 = piecewise5(t7, 0.0, t10, 0.0, t94);
        let t97 = piecewise5(t16, 0.0, t19, 0.0, t94);
        let t99 = t3 * t97 + t20 + 1.0;
        let t100 = param_B * t99;
        let t102 = t99 * t57;
        let t106 = 2.0 * t102 * t4 * t60 + t100 * t58;
        let t110 = piecewise3(t1, 0.0, t13 * t106 / 2.0 + t95 * t27 / 2.0);
        let t112 = piecewise5(t10, 0.0, t7, 0.0, -t94);
        let t115 = piecewise5(t19, 0.0, t16, 0.0, t4 - t73);
        let t117 = t115 * t3 + t36 + 1.0;
        let t118 = param_B * t117;
        let t120 = t117 * t79;
        let t124 = 2.0 * t120 * t4 * t82 + t118 * t80;
        let t128 = piecewise3(t31, 0.0, t112 * t43 / 2.0 + t33 * t124 / 2.0);
        let tvrho1 = t30 + t46 + t3 * (t110 + t128);
        vrho[ip * 2 + 1] += tvrho1;
        let t134 = 1.0 / t47 / t3;
        let t135 = t2 * t134;
        let t137 = -2.0 * t48 + 2.0 * t135;
        let t138 = piecewise5(t7, 0.0, t10, 0.0, t137);
        let t142 = piecewise5(t16, 0.0, t19, 0.0, t137);
        let t145 = t142 * t3 + 2.0 * t53;
        let t146 = param_B * t145;
        let t148 = t21 * t21;
        let t149 = 1.0 / t148;
        let t150 = t149 * t4;
        let t151 = t150 * t53;
        let t153 = t57 * t48;
        let t155 = t55 * t55;
        let t156 = param_C * t155;
        let t157 = t149 * t48;
        let t160 = t145 * t57;
        let t164 = t60 * t55;
        let t170 = 2.0 * t160 * t4 * t60 - 2.0 * t48 * t60 * t61 + t146 * t58 - 2.0 * t151 * t164 - t151 * t56 - t153 * t56 + 2.0 * t156 * t157;
        let t174 = piecewise3(t1, 0.0, t138 * t27 / 2.0 + t51 * t65 + t13 * t170 / 2.0);
        let t176 = piecewise5(t10, 0.0, t7, 0.0, -t137);
        let t180 = t34 * t134;
        let t183 = piecewise5(t19, 0.0, t16, 0.0, 2.0 * t48 + 2.0 * t180);
        let t186 = t183 * t3 + 2.0 * t75;
        let t187 = param_B * t186;
        let t189 = t37 * t37;
        let t190 = 1.0 / t189;
        let t191 = t190 * t4;
        let t192 = t191 * t75;
        let t194 = t79 * t48;
        let t196 = t77 * t77;
        let t197 = param_C * t196;
        let t198 = t190 * t48;
        let t201 = t186 * t79;
        let t205 = t82 * t77;
        let t211 = 2.0 * t201 * t4 * t82 - 2.0 * t48 * t82 * t83 + t187 * t80 - 2.0 * t192 * t205 - t192 * t78 - t194 * t78 + 2.0 * t197 * t198;
        let t215 = piecewise3(t31, 0.0, t176 * t43 / 2.0 + t71 * t87 + t33 * t211 / 2.0);
        let tv2rho20 = 2.0 * t69 + 2.0 * t91 + t3 * (t174 + t215);
        v2rho2[ip * 3] += tv2rho20;
        let t218 = 2.0 * t135;
        let t219 = piecewise5(t7, 0.0, t10, 0.0, t218);
        let t223 = piecewise5(t16, 0.0, t19, 0.0, t218);
        let t225 = t223 * t3 + t53 + t97;
        let t226 = param_B * t225;
        let t229 = t100 * t153;
        let t230 = param_C * t55;
        let t231 = t157 * t99;
        let t234 = t225 * t57;
        let t238 = t60 * t99;
        let t243 = 2.0 * t60 * t102 * t48;
        let t244 = 2.0 * t234 * t4 * t60 - t100 * t151 - 2.0 * t151 * t238 + t226 * t58 + 2.0 * t230 * t231 - t229 - t243;
        let t248 = piecewise3(t1, 0.0, t51 * t106 / 2.0 + t13 * t244 / 2.0 + t219 * t27 / 2.0 + t95 * t65 / 2.0);
        let t249 = piecewise5(t10, 0.0, t7, 0.0, -t218);
        let t254 = piecewise5(t19, 0.0, t16, 0.0, 2.0 * t180);
        let t256 = t254 * t3 + t115 + t75;
        let t257 = param_B * t256;
        let t260 = t118 * t194;
        let t261 = param_C * t77;
        let t262 = t198 * t117;
        let t265 = t256 * t79;
        let t269 = t82 * t117;
        let t274 = 2.0 * t82 * t120 * t48;
        let t275 = 2.0 * t265 * t4 * t82 - t118 * t192 - 2.0 * t192 * t269 + t257 * t80 + 2.0 * t261 * t262 - t260 - t274;
        let t279 = piecewise3(t31, 0.0, t112 * t87 / 2.0 + t71 * t124 / 2.0 + t249 * t43 / 2.0 + t33 * t275 / 2.0);
        let tv2rho21 = t69 + t91 + t110 + t128 + t3 * (t248 + t279);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t285 = 2.0 * t48 + 2.0 * t135;
        let t286 = piecewise5(t7, 0.0, t10, 0.0, t285);
        let t290 = piecewise5(t16, 0.0, t19, 0.0, t285);
        let t293 = t290 * t3 + 2.0 * t97;
        let t294 = param_B * t293;
        let t296 = t150 * t97;
        let t298 = t99 * t99;
        let t299 = param_C * t298;
        let t302 = t293 * t57;
        let t308 = 2.0 * t302 * t4 * t60 - t100 * t296 + 2.0 * t157 * t299 - 2.0 * t238 * t296 + t294 * t58 - t229 - t243;
        let t312 = piecewise3(t1, 0.0, t286 * t27 / 2.0 + t95 * t106 + t13 * t308 / 2.0);
        let t314 = piecewise5(t10, 0.0, t7, 0.0, -t285);
        let t320 = piecewise5(t19, 0.0, t16, 0.0, -2.0 * t48 + 2.0 * t180);
        let t323 = t3 * t320 + 2.0 * t115;
        let t324 = param_B * t323;
        let t326 = t191 * t115;
        let t328 = t117 * t117;
        let t329 = param_C * t328;
        let t332 = t323 * t79;
        let t338 = 2.0 * t332 * t4 * t82 - t118 * t326 + 2.0 * t198 * t329 - 2.0 * t269 * t326 + t324 * t80 - t260 - t274;
        let t342 = piecewise3(t31, 0.0, t314 * t43 / 2.0 + t112 * t124 + t33 * t338 / 2.0);
        let tv2rho22 = 2.0 * t110 + 2.0 * t128 + t3 * (t312 + t342);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t347 = t47 * t47;
        let t348 = 1.0 / t347;
        let t349 = t2 * t348;
        let t351 = 6.0 * t134 - 6.0 * t349;
        let t352 = piecewise5(t7, 0.0, t10, 0.0, t351);
        let t363 = t150 * t142;
        let t365 = t57 * t134;
        let t372 = 1.0 / t148 / t21;
        let t373 = t372 * t48;
        let t374 = t373 * t53;
        let t377 = t149 * t134;
        let t380 = piecewise5(t16, 0.0, t19, 0.0, t351);
        let t383 = t3 * t380 + 3.0 * t142;
        let t384 = t383 * t57;
        let t388 = param_B * t383;
        let t390 = t372 * t4;
        let t391 = t53 * t53;
        let t392 = t390 * t391;
        let t395 = t157 * t53;
        let t402 = t60 * t145;
        let t413 = 4.0 * t134 * t60 * t61 + 6.0 * t145 * t157 * t230 - 4.0 * t160 * t48 * t60 + 2.0 * t384 * t4 * t60 - 2.0 * t146 * t151 - 2.0 * t146 * t153 - 4.0 * t151 * t402 - 6.0 * t156 * t374 - 6.0 * t156 * t377 - 2.0 * t164 * t363 + 4.0 * t164 * t392 + 4.0 * t164 * t395 - t363 * t56 + 2.0 * t365 * t56 + t388 * t58 + 2.0 * t392 * t56 + 2.0 * t395 * t56;
        let t417 = piecewise3(t1, 0.0, t352 * t27 / 2.0 + 3.0 / 2.0 * t138 * t65 + 3.0 / 2.0 * t51 * t170 + t13 * t413 / 2.0);
        let t419 = piecewise5(t10, 0.0, t7, 0.0, -t351);
        let t430 = t191 * t183;
        let t432 = t79 * t134;
        let t439 = 1.0 / t189 / t37;
        let t440 = t439 * t48;
        let t441 = t440 * t75;
        let t444 = t190 * t134;
        let t447 = t34 * t348;
        let t450 = piecewise5(t19, 0.0, t16, 0.0, -6.0 * t134 - 6.0 * t447);
        let t453 = t3 * t450 + 3.0 * t183;
        let t454 = t453 * t79;
        let t458 = param_B * t453;
        let t460 = t439 * t4;
        let t461 = t75 * t75;
        let t462 = t460 * t461;
        let t465 = t198 * t75;
        let t472 = t82 * t186;
        let t483 = 4.0 * t134 * t82 * t83 + 6.0 * t186 * t198 * t261 - 4.0 * t201 * t48 * t82 + 2.0 * t4 * t454 * t82 - 2.0 * t187 * t192 - 2.0 * t187 * t194 - 4.0 * t192 * t472 - 6.0 * t197 * t441 - 6.0 * t197 * t444 - 2.0 * t205 * t430 + 4.0 * t205 * t462 + 4.0 * t205 * t465 - t430 * t78 + 2.0 * t432 * t78 + t458 * t80 + 2.0 * t462 * t78 + 2.0 * t465 * t78;
        let t487 = piecewise3(t31, 0.0, t419 * t43 / 2.0 + 3.0 / 2.0 * t176 * t87 + 3.0 / 2.0 * t71 * t211 + t33 * t483 / 2.0);
        let tv3rho30 = 3.0 * t174 + 3.0 * t215 + t3 * (t417 + t487);
        v3rho3[ip * 4] += tv3rho30;
        let t490 = 2.0 * t248;
        let t491 = 2.0 * t279;
        let t492 = 2.0 * t134;
        let t493 = 6.0 * t349;
        let t494 = t492 - t493;
        let t495 = piecewise5(t7, 0.0, t10, 0.0, t494);
        let t506 = t226 * t153;
        let t510 = 2.0 * t100 * t365;
        let t511 = param_C * t145;
        let t514 = t157 * t225;
        let t517 = piecewise5(t16, 0.0, t19, 0.0, t494);
        let t519 = 2.0 * t223;
        let t520 = t3 * t517 + t142 + t519;
        let t521 = t520 * t57;
        let t525 = param_B * t520;
        let t529 = t238 * t395;
        let t533 = t100 * t395;
        let t535 = t230 * t372;
        let t536 = t48 * t99;
        let t537 = t536 * t53;
        let t540 = t377 * t99;
        let t541 = t230 * t540;
        let t543 = t60 * t225;
        let t547 = t60 * t234 * t48;
        let t553 = 4.0 * t60 * t102 * t134;
        let t554 = 2.0 * t4 * t521 * t60 - t100 * t363 + 2.0 * t100 * t392 - 2.0 * t151 * t226 - 4.0 * t151 * t543 + 4.0 * t230 * t514 + 2.0 * t231 * t511 - 2.0 * t238 * t363 + 4.0 * t238 * t392 + t525 * t58 - 6.0 * t535 * t537 - 2.0 * t506 + t510 + 4.0 * t529 + 2.0 * t533 - 6.0 * t541 - 4.0 * t547 + t553;
        let t558 = piecewise3(t1, 0.0, t495 * t27 / 2.0 + t219 * t65 + t95 * t170 / 2.0 + t138 * t106 / 2.0 + t51 * t244 + t13 * t554 / 2.0);
        let t560 = piecewise5(t10, 0.0, t7, 0.0, -t494);
        let t571 = t257 * t194;
        let t575 = 2.0 * t118 * t432;
        let t576 = param_C * t186;
        let t579 = t198 * t256;
        let t582 = 6.0 * t447;
        let t584 = piecewise5(t19, 0.0, t16, 0.0, -t492 - t582);
        let t586 = 2.0 * t254;
        let t587 = t3 * t584 + t183 + t586;
        let t588 = t587 * t79;
        let t592 = param_B * t587;
        let t596 = t269 * t465;
        let t600 = t118 * t465;
        let t602 = t261 * t439;
        let t603 = t48 * t117;
        let t604 = t603 * t75;
        let t607 = t444 * t117;
        let t608 = t261 * t607;
        let t610 = t82 * t256;
        let t614 = t82 * t265 * t48;
        let t620 = 4.0 * t82 * t120 * t134;
        let t621 = 2.0 * t4 * t588 * t82 - t118 * t430 + 2.0 * t118 * t462 - 2.0 * t192 * t257 - 4.0 * t192 * t610 + 4.0 * t261 * t579 + 2.0 * t262 * t576 - 2.0 * t269 * t430 + 4.0 * t269 * t462 + t592 * t80 - 6.0 * t602 * t604 - 2.0 * t571 + t575 + 4.0 * t596 + 2.0 * t600 - 6.0 * t608 - 4.0 * t614 + t620;
        let t625 = piecewise3(t31, 0.0, t560 * t43 / 2.0 + t249 * t87 + t112 * t211 / 2.0 + t176 * t124 / 2.0 + t71 * t275 + t33 * t621 / 2.0);
        let tv3rho31 = t174 + t215 + t490 + t491 + t3 * (t558 + t625);
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t628 = -t492 - t493;
        let t629 = piecewise5(t7, 0.0, t10, 0.0, t628);
        let t642 = t150 * t223;
        let t644 = param_C * t99;
        let t649 = piecewise5(t16, 0.0, t19, 0.0, t628);
        let t651 = t3 * t649 + t290 + t519;
        let t652 = t651 * t57;
        let t656 = t157 * t293;
        let t659 = 2.0 * t4 * t60 * t652 - t100 * t642 - t151 * t294 - t226 * t296 + 2.0 * t230 * t656 - 4.0 * t299 * t374 + 4.0 * t514 * t644 - t506 + t510 + t533 - 2.0 * t541 - 2.0 * t547 + t553;
        let t660 = t294 * t153;
        let t661 = t299 * t377;
        let t663 = param_B * t651;
        let t665 = t97 * t53;
        let t666 = t390 * t665;
        let t669 = t157 * t97;
        let t670 = t100 * t669;
        let t672 = t60 * t302 * t48;
        let t675 = t536 * t97;
        let t678 = t238 * t669;
        let t680 = t100 * t372;
        let t681 = t4 * t97;
        let t682 = t681 * t53;
        let t685 = t60 * t293;
        let t692 = -2.0 * t151 * t685 - 2.0 * t238 * t642 + 4.0 * t238 * t666 - 2.0 * t296 * t543 - 2.0 * t535 * t675 + t58 * t663 + 2.0 * t680 * t682 + 2.0 * t529 - t660 - 4.0 * t661 + t670 - 2.0 * t672 + 2.0 * t678;
        let t693 = t659 + t692;
        let t697 = piecewise3(t1, 0.0, t629 * t27 / 2.0 + t286 * t65 / 2.0 + t219 * t106 + t95 * t244 + t51 * t308 / 2.0 + t13 * t693 / 2.0);
        let t699 = piecewise5(t10, 0.0, t7, 0.0, -t628);
        let t710 = t324 * t194;
        let t711 = t329 * t444;
        let t714 = piecewise5(t19, 0.0, t16, 0.0, t492 - t582);
        let t716 = t3 * t714 + t320 + t586;
        let t717 = param_B * t716;
        let t719 = t115 * t75;
        let t720 = t460 * t719;
        let t725 = t191 * t254;
        let t727 = -t118 * t725 - t192 * t324 - t257 * t326 + 4.0 * t269 * t720 + t717 * t80 - t571 + t575 + t600 - 2.0 * t608 - 2.0 * t614 + t620 - t710 - 4.0 * t711;
        let t728 = param_C * t117;
        let t733 = t716 * t79;
        let t737 = t198 * t323;
        let t740 = t198 * t115;
        let t741 = t118 * t740;
        let t743 = t82 * t332 * t48;
        let t746 = t603 * t115;
        let t749 = t269 * t740;
        let t751 = t118 * t439;
        let t752 = t4 * t115;
        let t753 = t752 * t75;
        let t756 = t82 * t323;
        let t763 = 2.0 * t4 * t733 * t82 - 2.0 * t192 * t756 + 2.0 * t261 * t737 - 2.0 * t269 * t725 - 2.0 * t326 * t610 - 4.0 * t329 * t441 + 4.0 * t579 * t728 - 2.0 * t602 * t746 + 2.0 * t751 * t753 + 2.0 * t596 + t741 - 2.0 * t743 + 2.0 * t749;
        let t764 = t727 + t763;
        let t768 = piecewise3(t31, 0.0, t699 * t43 / 2.0 + t314 * t87 / 2.0 + t249 * t124 + t112 * t275 + t71 * t338 / 2.0 + t33 * t764 / 2.0);
        let tv3rho32 = t490 + t491 + t312 + t342 + t3 * (t697 + t768);
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t774 = -6.0 * t134 - 6.0 * t349;
        let t775 = piecewise5(t7, 0.0, t10, 0.0, t774);
        let t786 = t150 * t290;
        let t793 = piecewise5(t16, 0.0, t19, 0.0, t774);
        let t796 = t3 * t793 + 3.0 * t290;
        let t797 = t796 * t57;
        let t801 = param_B * t796;
        let t804 = t97 * t97;
        let t805 = t390 * t804;
        let t816 = -6.0 * t299 * t373 * t97 + 2.0 * t4 * t60 * t797 - t100 * t786 + 2.0 * t100 * t805 - 2.0 * t238 * t786 + 4.0 * t238 * t805 - 2.0 * t294 * t296 - 4.0 * t296 * t685 + t58 * t801 + 6.0 * t644 * t656 + t510 + t553 - 2.0 * t660 - 6.0 * t661 + 2.0 * t670 - 4.0 * t672 + 4.0 * t678;
        let t820 = piecewise3(t1, 0.0, t775 * t27 / 2.0 + 3.0 / 2.0 * t286 * t106 + 3.0 / 2.0 * t95 * t308 + t13 * t816 / 2.0);
        let t822 = piecewise5(t10, 0.0, t7, 0.0, -t774);
        let t833 = t191 * t320;
        let t839 = piecewise5(t19, 0.0, t16, 0.0, 6.0 * t134 - 6.0 * t447);
        let t842 = t3 * t839 + 3.0 * t320;
        let t843 = param_B * t842;
        let t846 = t115 * t115;
        let t847 = t460 * t846;
        let t861 = t842 * t79;
        let t865 = -6.0 * t115 * t329 * t440 + 2.0 * t4 * t82 * t861 - t118 * t833 + 2.0 * t118 * t847 - 2.0 * t269 * t833 + 4.0 * t269 * t847 - 2.0 * t324 * t326 - 4.0 * t326 * t756 + 6.0 * t728 * t737 + t80 * t843 + t575 + t620 - 2.0 * t710 - 6.0 * t711 + 2.0 * t741 - 4.0 * t743 + 4.0 * t749;
        let t869 = piecewise3(t31, 0.0, t822 * t43 / 2.0 + 3.0 / 2.0 * t314 * t124 + 3.0 / 2.0 * t112 * t338 + t33 * t865 / 2.0);
        let tv3rho33 = 3.0 * t312 + 3.0 * t342 + t3 * (t820 + t869);
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

/// LDA_K_GDS08_WORKER lxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = 1.0 - t5 <= zeta_threshold;
        let t11 = -t8;
        let t12 = piecewise5(t7, t8, t10, t11, t5);
        let t13 = 1.0 + t12;
        let t16 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t19 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t20 = piecewise5(t16, t8, t19, t11, t5);
        let t21 = 1.0 + t20;
        let t23 = f64::ln(t21 * t3);
        let t25 = t23 * t23;
        let t27 = t23 * param_B + t25 * param_C + param_A;
        let t30 = piecewise3(t1, 0.0, t13 * t27 / 2.0);
        let t31 = rho1 <= dens_threshold;
        let t32 = piecewise5(t10, t8, t7, t11, -t5);
        let t33 = 1.0 + t32;
        let t34 = -t2;
        let t36 = piecewise5(t19, t8, t16, t11, t34 * t4);
        let t37 = 1.0 + t36;
        let t39 = f64::ln(t37 * t3);
        let t41 = t39 * t39;
        let t43 = t39 * param_B + t41 * param_C + param_A;
        let t46 = piecewise3(t31, 0.0, t33 * t43 / 2.0);
        let tzk0 = t30 + t46;
        zk[ip] += tzk0;
        let t47 = t3 * t3;
        let t48 = 1.0 / t47;
        let t49 = t2 * t48;
        let t50 = t4 - t49;
        let t51 = piecewise5(t7, 0.0, t10, 0.0, t50);
        let t53 = piecewise5(t16, 0.0, t19, 0.0, t50);
        let t55 = t3 * t53 + t20 + 1.0;
        let t56 = param_B * t55;
        let t57 = 1.0 / t21;
        let t58 = t57 * t4;
        let t60 = param_C * t23;
        let t61 = t55 * t57;
        let t65 = 2.0 * t4 * t60 * t61 + t56 * t58;
        let t69 = piecewise3(t1, 0.0, t13 * t65 / 2.0 + t51 * t27 / 2.0);
        let t71 = piecewise5(t10, 0.0, t7, 0.0, -t50);
        let t73 = t34 * t48;
        let t75 = piecewise5(t19, 0.0, t16, 0.0, -t4 - t73);
        let t77 = t3 * t75 + t36 + 1.0;
        let t78 = param_B * t77;
        let t79 = 1.0 / t37;
        let t80 = t79 * t4;
        let t82 = param_C * t39;
        let t83 = t77 * t79;
        let t87 = 2.0 * t4 * t82 * t83 + t78 * t80;
        let t91 = piecewise3(t31, 0.0, t33 * t87 / 2.0 + t71 * t43 / 2.0);
        let tvrho0 = t30 + t46 + t3 * (t69 + t91);
        vrho[ip * 2] += tvrho0;
        let t94 = -t4 - t49;
        let t95 = piecewise5(t7, 0.0, t10, 0.0, t94);
        let t97 = piecewise5(t16, 0.0, t19, 0.0, t94);
        let t99 = t3 * t97 + t20 + 1.0;
        let t100 = param_B * t99;
        let t102 = t99 * t57;
        let t106 = 2.0 * t102 * t4 * t60 + t100 * t58;
        let t110 = piecewise3(t1, 0.0, t13 * t106 / 2.0 + t95 * t27 / 2.0);
        let t112 = piecewise5(t10, 0.0, t7, 0.0, -t94);
        let t115 = piecewise5(t19, 0.0, t16, 0.0, t4 - t73);
        let t117 = t115 * t3 + t36 + 1.0;
        let t118 = param_B * t117;
        let t120 = t117 * t79;
        let t124 = 2.0 * t120 * t4 * t82 + t118 * t80;
        let t128 = piecewise3(t31, 0.0, t112 * t43 / 2.0 + t33 * t124 / 2.0);
        let tvrho1 = t30 + t46 + t3 * (t110 + t128);
        vrho[ip * 2 + 1] += tvrho1;
        let t134 = 1.0 / t47 / t3;
        let t135 = t2 * t134;
        let t137 = -2.0 * t48 + 2.0 * t135;
        let t138 = piecewise5(t7, 0.0, t10, 0.0, t137);
        let t142 = piecewise5(t16, 0.0, t19, 0.0, t137);
        let t145 = t142 * t3 + 2.0 * t53;
        let t146 = param_B * t145;
        let t148 = t21 * t21;
        let t149 = 1.0 / t148;
        let t150 = t149 * t4;
        let t151 = t150 * t53;
        let t153 = t57 * t48;
        let t155 = t55 * t55;
        let t156 = param_C * t155;
        let t157 = t149 * t48;
        let t160 = t145 * t57;
        let t164 = t60 * t55;
        let t170 = 2.0 * t160 * t4 * t60 - 2.0 * t48 * t60 * t61 + t146 * t58 - 2.0 * t151 * t164 - t151 * t56 - t153 * t56 + 2.0 * t156 * t157;
        let t174 = piecewise3(t1, 0.0, t138 * t27 / 2.0 + t51 * t65 + t13 * t170 / 2.0);
        let t176 = piecewise5(t10, 0.0, t7, 0.0, -t137);
        let t180 = t34 * t134;
        let t183 = piecewise5(t19, 0.0, t16, 0.0, 2.0 * t48 + 2.0 * t180);
        let t186 = t183 * t3 + 2.0 * t75;
        let t187 = param_B * t186;
        let t189 = t37 * t37;
        let t190 = 1.0 / t189;
        let t191 = t190 * t4;
        let t192 = t191 * t75;
        let t194 = t79 * t48;
        let t196 = t77 * t77;
        let t197 = param_C * t196;
        let t198 = t190 * t48;
        let t201 = t186 * t79;
        let t205 = t82 * t77;
        let t211 = 2.0 * t201 * t4 * t82 - 2.0 * t48 * t82 * t83 + t187 * t80 - 2.0 * t192 * t205 - t192 * t78 - t194 * t78 + 2.0 * t197 * t198;
        let t215 = piecewise3(t31, 0.0, t176 * t43 / 2.0 + t71 * t87 + t33 * t211 / 2.0);
        let tv2rho20 = 2.0 * t69 + 2.0 * t91 + t3 * (t174 + t215);
        v2rho2[ip * 3] += tv2rho20;
        let t218 = 2.0 * t135;
        let t219 = piecewise5(t7, 0.0, t10, 0.0, t218);
        let t223 = piecewise5(t16, 0.0, t19, 0.0, t218);
        let t225 = t223 * t3 + t53 + t97;
        let t226 = param_B * t225;
        let t229 = t100 * t153;
        let t230 = param_C * t55;
        let t231 = t157 * t99;
        let t234 = t225 * t57;
        let t238 = t60 * t99;
        let t243 = 2.0 * t60 * t102 * t48;
        let t244 = 2.0 * t234 * t4 * t60 - t100 * t151 - 2.0 * t151 * t238 + t226 * t58 + 2.0 * t230 * t231 - t229 - t243;
        let t248 = piecewise3(t1, 0.0, t51 * t106 / 2.0 + t13 * t244 / 2.0 + t219 * t27 / 2.0 + t95 * t65 / 2.0);
        let t249 = piecewise5(t10, 0.0, t7, 0.0, -t218);
        let t254 = piecewise5(t19, 0.0, t16, 0.0, 2.0 * t180);
        let t256 = t254 * t3 + t115 + t75;
        let t257 = param_B * t256;
        let t260 = t118 * t194;
        let t261 = param_C * t77;
        let t262 = t198 * t117;
        let t265 = t256 * t79;
        let t269 = t82 * t117;
        let t274 = 2.0 * t82 * t120 * t48;
        let t275 = 2.0 * t265 * t4 * t82 - t118 * t192 - 2.0 * t192 * t269 + t257 * t80 + 2.0 * t261 * t262 - t260 - t274;
        let t279 = piecewise3(t31, 0.0, t112 * t87 / 2.0 + t71 * t124 / 2.0 + t249 * t43 / 2.0 + t33 * t275 / 2.0);
        let tv2rho21 = t69 + t91 + t110 + t128 + t3 * (t248 + t279);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t285 = 2.0 * t48 + 2.0 * t135;
        let t286 = piecewise5(t7, 0.0, t10, 0.0, t285);
        let t290 = piecewise5(t16, 0.0, t19, 0.0, t285);
        let t293 = t290 * t3 + 2.0 * t97;
        let t294 = param_B * t293;
        let t296 = t150 * t97;
        let t298 = t99 * t99;
        let t299 = param_C * t298;
        let t302 = t293 * t57;
        let t308 = 2.0 * t302 * t4 * t60 - t100 * t296 + 2.0 * t157 * t299 - 2.0 * t238 * t296 + t294 * t58 - t229 - t243;
        let t312 = piecewise3(t1, 0.0, t286 * t27 / 2.0 + t95 * t106 + t13 * t308 / 2.0);
        let t314 = piecewise5(t10, 0.0, t7, 0.0, -t285);
        let t320 = piecewise5(t19, 0.0, t16, 0.0, -2.0 * t48 + 2.0 * t180);
        let t323 = t3 * t320 + 2.0 * t115;
        let t324 = param_B * t323;
        let t326 = t191 * t115;
        let t328 = t117 * t117;
        let t329 = param_C * t328;
        let t332 = t323 * t79;
        let t338 = 2.0 * t332 * t4 * t82 - t118 * t326 + 2.0 * t198 * t329 - 2.0 * t269 * t326 + t324 * t80 - t260 - t274;
        let t342 = piecewise3(t31, 0.0, t314 * t43 / 2.0 + t112 * t124 + t33 * t338 / 2.0);
        let tv2rho22 = 2.0 * t110 + 2.0 * t128 + t3 * (t312 + t342);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t347 = t47 * t47;
        let t348 = 1.0 / t347;
        let t349 = t2 * t348;
        let t351 = 6.0 * t134 - 6.0 * t349;
        let t352 = piecewise5(t7, 0.0, t10, 0.0, t351);
        let t363 = t150 * t142;
        let t365 = t57 * t134;
        let t372 = 1.0 / t148 / t21;
        let t373 = t372 * t48;
        let t374 = t373 * t53;
        let t377 = t149 * t134;
        let t380 = piecewise5(t16, 0.0, t19, 0.0, t351);
        let t383 = t3 * t380 + 3.0 * t142;
        let t384 = t383 * t57;
        let t388 = param_B * t383;
        let t390 = t372 * t4;
        let t391 = t53 * t53;
        let t392 = t390 * t391;
        let t395 = t157 * t53;
        let t402 = t60 * t145;
        let t413 = 4.0 * t134 * t60 * t61 + 6.0 * t145 * t157 * t230 - 4.0 * t160 * t48 * t60 + 2.0 * t384 * t4 * t60 - 2.0 * t146 * t151 - 2.0 * t146 * t153 - 4.0 * t151 * t402 - 6.0 * t156 * t374 - 6.0 * t156 * t377 - 2.0 * t164 * t363 + 4.0 * t164 * t392 + 4.0 * t164 * t395 - t363 * t56 + 2.0 * t365 * t56 + t388 * t58 + 2.0 * t392 * t56 + 2.0 * t395 * t56;
        let t417 = piecewise3(t1, 0.0, t352 * t27 / 2.0 + 3.0 / 2.0 * t138 * t65 + 3.0 / 2.0 * t51 * t170 + t13 * t413 / 2.0);
        let t419 = piecewise5(t10, 0.0, t7, 0.0, -t351);
        let t430 = t191 * t183;
        let t432 = t79 * t134;
        let t439 = 1.0 / t189 / t37;
        let t440 = t439 * t48;
        let t441 = t440 * t75;
        let t444 = t190 * t134;
        let t447 = t34 * t348;
        let t450 = piecewise5(t19, 0.0, t16, 0.0, -6.0 * t134 - 6.0 * t447);
        let t453 = t3 * t450 + 3.0 * t183;
        let t454 = t453 * t79;
        let t458 = param_B * t453;
        let t460 = t439 * t4;
        let t461 = t75 * t75;
        let t462 = t460 * t461;
        let t465 = t198 * t75;
        let t472 = t82 * t186;
        let t483 = 4.0 * t134 * t82 * t83 + 6.0 * t186 * t198 * t261 - 4.0 * t201 * t48 * t82 + 2.0 * t4 * t454 * t82 - 2.0 * t187 * t192 - 2.0 * t187 * t194 - 4.0 * t192 * t472 - 6.0 * t197 * t441 - 6.0 * t197 * t444 - 2.0 * t205 * t430 + 4.0 * t205 * t462 + 4.0 * t205 * t465 - t430 * t78 + 2.0 * t432 * t78 + t458 * t80 + 2.0 * t462 * t78 + 2.0 * t465 * t78;
        let t487 = piecewise3(t31, 0.0, t419 * t43 / 2.0 + 3.0 / 2.0 * t176 * t87 + 3.0 / 2.0 * t71 * t211 + t33 * t483 / 2.0);
        let tv3rho30 = 3.0 * t174 + 3.0 * t215 + t3 * (t417 + t487);
        v3rho3[ip * 4] += tv3rho30;
        let t490 = 2.0 * t248;
        let t491 = 2.0 * t279;
        let t492 = 2.0 * t134;
        let t493 = 6.0 * t349;
        let t494 = t492 - t493;
        let t495 = piecewise5(t7, 0.0, t10, 0.0, t494);
        let t506 = t226 * t153;
        let t510 = 2.0 * t100 * t365;
        let t511 = param_C * t145;
        let t514 = t157 * t225;
        let t517 = piecewise5(t16, 0.0, t19, 0.0, t494);
        let t519 = 2.0 * t223;
        let t520 = t3 * t517 + t142 + t519;
        let t521 = t520 * t57;
        let t525 = param_B * t520;
        let t529 = t238 * t395;
        let t533 = t100 * t395;
        let t535 = t230 * t372;
        let t536 = t48 * t99;
        let t537 = t536 * t53;
        let t540 = t377 * t99;
        let t541 = t230 * t540;
        let t543 = t60 * t225;
        let t547 = t60 * t234 * t48;
        let t553 = 4.0 * t60 * t102 * t134;
        let t554 = 2.0 * t4 * t521 * t60 - t100 * t363 + 2.0 * t100 * t392 - 2.0 * t151 * t226 - 4.0 * t151 * t543 + 4.0 * t230 * t514 + 2.0 * t231 * t511 - 2.0 * t238 * t363 + 4.0 * t238 * t392 + t525 * t58 - 6.0 * t535 * t537 - 2.0 * t506 + t510 + 4.0 * t529 + 2.0 * t533 - 6.0 * t541 - 4.0 * t547 + t553;
        let t558 = piecewise3(t1, 0.0, t495 * t27 / 2.0 + t219 * t65 + t95 * t170 / 2.0 + t138 * t106 / 2.0 + t51 * t244 + t13 * t554 / 2.0);
        let t560 = piecewise5(t10, 0.0, t7, 0.0, -t494);
        let t571 = t257 * t194;
        let t575 = 2.0 * t118 * t432;
        let t576 = param_C * t186;
        let t579 = t198 * t256;
        let t582 = 6.0 * t447;
        let t584 = piecewise5(t19, 0.0, t16, 0.0, -t492 - t582);
        let t586 = 2.0 * t254;
        let t587 = t3 * t584 + t183 + t586;
        let t588 = t587 * t79;
        let t592 = param_B * t587;
        let t596 = t269 * t465;
        let t600 = t118 * t465;
        let t602 = t261 * t439;
        let t603 = t48 * t117;
        let t604 = t603 * t75;
        let t607 = t444 * t117;
        let t608 = t261 * t607;
        let t610 = t82 * t256;
        let t614 = t82 * t265 * t48;
        let t620 = 4.0 * t82 * t120 * t134;
        let t621 = 2.0 * t4 * t588 * t82 - t118 * t430 + 2.0 * t118 * t462 - 2.0 * t192 * t257 - 4.0 * t192 * t610 + 4.0 * t261 * t579 + 2.0 * t262 * t576 - 2.0 * t269 * t430 + 4.0 * t269 * t462 + t592 * t80 - 6.0 * t602 * t604 - 2.0 * t571 + t575 + 4.0 * t596 + 2.0 * t600 - 6.0 * t608 - 4.0 * t614 + t620;
        let t625 = piecewise3(t31, 0.0, t560 * t43 / 2.0 + t249 * t87 + t112 * t211 / 2.0 + t176 * t124 / 2.0 + t71 * t275 + t33 * t621 / 2.0);
        let tv3rho31 = t174 + t215 + t490 + t491 + t3 * (t558 + t625);
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t628 = -t492 - t493;
        let t629 = piecewise5(t7, 0.0, t10, 0.0, t628);
        let t642 = t150 * t223;
        let t644 = param_C * t99;
        let t649 = piecewise5(t16, 0.0, t19, 0.0, t628);
        let t651 = t3 * t649 + t290 + t519;
        let t652 = t651 * t57;
        let t656 = t157 * t293;
        let t659 = 2.0 * t4 * t60 * t652 - t100 * t642 - t151 * t294 - t226 * t296 + 2.0 * t230 * t656 - 4.0 * t299 * t374 + 4.0 * t514 * t644 - t506 + t510 + t533 - 2.0 * t541 - 2.0 * t547 + t553;
        let t660 = t294 * t153;
        let t661 = t299 * t377;
        let t663 = param_B * t651;
        let t665 = t97 * t53;
        let t666 = t390 * t665;
        let t669 = t157 * t97;
        let t670 = t100 * t669;
        let t672 = t60 * t302 * t48;
        let t675 = t536 * t97;
        let t678 = t238 * t669;
        let t680 = t100 * t372;
        let t681 = t4 * t97;
        let t682 = t681 * t53;
        let t685 = t60 * t293;
        let t692 = -2.0 * t151 * t685 - 2.0 * t238 * t642 + 4.0 * t238 * t666 - 2.0 * t296 * t543 - 2.0 * t535 * t675 + t58 * t663 + 2.0 * t680 * t682 + 2.0 * t529 - t660 - 4.0 * t661 + t670 - 2.0 * t672 + 2.0 * t678;
        let t693 = t659 + t692;
        let t697 = piecewise3(t1, 0.0, t629 * t27 / 2.0 + t286 * t65 / 2.0 + t219 * t106 + t95 * t244 + t51 * t308 / 2.0 + t13 * t693 / 2.0);
        let t699 = piecewise5(t10, 0.0, t7, 0.0, -t628);
        let t710 = t324 * t194;
        let t711 = t329 * t444;
        let t714 = piecewise5(t19, 0.0, t16, 0.0, t492 - t582);
        let t716 = t3 * t714 + t320 + t586;
        let t717 = param_B * t716;
        let t719 = t115 * t75;
        let t720 = t460 * t719;
        let t725 = t191 * t254;
        let t727 = -t118 * t725 - t192 * t324 - t257 * t326 + 4.0 * t269 * t720 + t717 * t80 - t571 + t575 + t600 - 2.0 * t608 - 2.0 * t614 + t620 - t710 - 4.0 * t711;
        let t728 = param_C * t117;
        let t733 = t716 * t79;
        let t737 = t198 * t323;
        let t740 = t198 * t115;
        let t741 = t118 * t740;
        let t743 = t82 * t332 * t48;
        let t746 = t603 * t115;
        let t749 = t269 * t740;
        let t751 = t118 * t439;
        let t752 = t4 * t115;
        let t753 = t752 * t75;
        let t756 = t82 * t323;
        let t763 = 2.0 * t4 * t733 * t82 - 2.0 * t192 * t756 + 2.0 * t261 * t737 - 2.0 * t269 * t725 - 2.0 * t326 * t610 - 4.0 * t329 * t441 + 4.0 * t579 * t728 - 2.0 * t602 * t746 + 2.0 * t751 * t753 + 2.0 * t596 + t741 - 2.0 * t743 + 2.0 * t749;
        let t764 = t727 + t763;
        let t768 = piecewise3(t31, 0.0, t699 * t43 / 2.0 + t314 * t87 / 2.0 + t249 * t124 + t112 * t275 + t71 * t338 / 2.0 + t33 * t764 / 2.0);
        let tv3rho32 = t490 + t491 + t312 + t342 + t3 * (t697 + t768);
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t774 = -6.0 * t134 - 6.0 * t349;
        let t775 = piecewise5(t7, 0.0, t10, 0.0, t774);
        let t786 = t150 * t290;
        let t793 = piecewise5(t16, 0.0, t19, 0.0, t774);
        let t796 = t3 * t793 + 3.0 * t290;
        let t797 = t796 * t57;
        let t801 = param_B * t796;
        let t804 = t97 * t97;
        let t805 = t390 * t804;
        let t816 = -6.0 * t299 * t373 * t97 + 2.0 * t4 * t60 * t797 - t100 * t786 + 2.0 * t100 * t805 - 2.0 * t238 * t786 + 4.0 * t238 * t805 - 2.0 * t294 * t296 - 4.0 * t296 * t685 + t58 * t801 + 6.0 * t644 * t656 + t510 + t553 - 2.0 * t660 - 6.0 * t661 + 2.0 * t670 - 4.0 * t672 + 4.0 * t678;
        let t820 = piecewise3(t1, 0.0, t775 * t27 / 2.0 + 3.0 / 2.0 * t286 * t106 + 3.0 / 2.0 * t95 * t308 + t13 * t816 / 2.0);
        let t822 = piecewise5(t10, 0.0, t7, 0.0, -t774);
        let t833 = t191 * t320;
        let t839 = piecewise5(t19, 0.0, t16, 0.0, 6.0 * t134 - 6.0 * t447);
        let t842 = t3 * t839 + 3.0 * t320;
        let t843 = param_B * t842;
        let t846 = t115 * t115;
        let t847 = t460 * t846;
        let t861 = t842 * t79;
        let t865 = -6.0 * t115 * t329 * t440 + 2.0 * t4 * t82 * t861 - t118 * t833 + 2.0 * t118 * t847 - 2.0 * t269 * t833 + 4.0 * t269 * t847 - 2.0 * t324 * t326 - 4.0 * t326 * t756 + 6.0 * t728 * t737 + t80 * t843 + t575 + t620 - 2.0 * t710 - 6.0 * t711 + 2.0 * t741 - 4.0 * t743 + 4.0 * t749;
        let t869 = piecewise3(t31, 0.0, t822 * t43 / 2.0 + 3.0 / 2.0 * t314 * t124 + 3.0 / 2.0 * t112 * t338 + t33 * t865 / 2.0);
        let tv3rho33 = 3.0 * t312 + 3.0 * t342 + t3 * (t820 + t869);
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t875 = 1.0 / t347 / t3;
        let t876 = t2 * t875;
        let t878 = -24.0 * t348 + 24.0 * t876;
        let t879 = piecewise5(t7, 0.0, t10, 0.0, t878);
        let t890 = piecewise5(t16, 0.0, t19, 0.0, t878);
        let t893 = t3 * t890 + 4.0 * t380;
        let t898 = t57 * t348;
        let t901 = t145 * t145;
        let t905 = t149 * t348;
        let t909 = t390 * t53 * t142;
        let t916 = t157 * t142;
        let t922 = t148 * t148;
        let t923 = 1.0 / t922;
        let t924 = t923 * t48;
        let t925 = t924 * t391;
        let t928 = t372 * t134;
        let t929 = t928 * t53;
        let t935 = t923 * t4;
        let t937 = t935 * t391 * t53;
        let t940 = t373 * t391;
        let t949 = -3.0 * t388 * t153 + param_B * t893 * t58 + 6.0 * t146 * t365 - 6.0 * t56 * t898 + 6.0 * param_C * t901 * t157 + 22.0 * t156 * t905 + 12.0 * t164 * t909 + 6.0 * t146 * t392 + 6.0 * t146 * t395 + 3.0 * t56 * t916 - 28.0 * t230 * t377 * t145 + 22.0 * t156 * t925 + 28.0 * t156 * t929 - 6.0 * t60 * t384 * t48 - 6.0 * t56 * t937 - 6.0 * t56 * t940 + 12.0 * t60 * t160 * t134 - 12.0 * t60 * t61 * t348;
        let t954 = t150 * t380;
        let t956 = t377 * t53;
        let t962 = t373 * t142;
        let t977 = t4 * t142 * t53;
        let t997 = -28.0 * t145 * t48 * t53 * t535 + 2.0 * t4 * t57 * t60 * t893 - 6.0 * t151 * t383 * t60 + 8.0 * t157 * t230 * t383 + 6.0 * t372 * t56 * t977 - 3.0 * t146 * t363 - 3.0 * t151 * t388 - 8.0 * t156 * t962 + 6.0 * t164 * t916 - 12.0 * t164 * t937 - 12.0 * t164 * t940 - 2.0 * t164 * t954 - 12.0 * t164 * t956 - 6.0 * t363 * t402 + 12.0 * t392 * t402 + 12.0 * t395 * t402 - t56 * t954 - 6.0 * t56 * t956;
        let t1002 = piecewise3(t1, 0.0, t879 * t27 / 2.0 + 2.0 * t352 * t65 + 3.0 * t138 * t170 + 2.0 * t51 * t413 + t13 * (t949 + t997) / 2.0);
        let t1004 = piecewise5(t10, 0.0, t7, 0.0, -t878);
        let t1015 = t34 * t875;
        let t1018 = piecewise5(t19, 0.0, t16, 0.0, 24.0 * t348 + 24.0 * t1015);
        let t1021 = t1018 * t3 + 4.0 * t450;
        let t1026 = t79 * t348;
        let t1029 = t186 * t186;
        let t1033 = t190 * t348;
        let t1037 = t460 * t75 * t183;
        let t1044 = t198 * t183;
        let t1050 = t189 * t189;
        let t1051 = 1.0 / t1050;
        let t1052 = t1051 * t48;
        let t1053 = t1052 * t461;
        let t1056 = t439 * t134;
        let t1057 = t1056 * t75;
        let t1063 = t1051 * t4;
        let t1065 = t1063 * t461 * t75;
        let t1068 = t440 * t461;
        let t1077 = -3.0 * t458 * t194 + param_B * t1021 * t80 + 6.0 * t187 * t432 - 6.0 * t78 * t1026 + 6.0 * param_C * t1029 * t198 + 22.0 * t197 * t1033 + 12.0 * t205 * t1037 + 6.0 * t187 * t462 + 6.0 * t187 * t465 + 3.0 * t78 * t1044 - 28.0 * t261 * t444 * t186 + 22.0 * t197 * t1053 + 28.0 * t197 * t1057 - 6.0 * t82 * t454 * t48 - 6.0 * t78 * t1065 - 6.0 * t78 * t1068 + 12.0 * t82 * t201 * t134 - 12.0 * t82 * t83 * t348;
        let t1082 = t191 * t450;
        let t1084 = t444 * t75;
        let t1090 = t440 * t183;
        let t1105 = t4 * t183 * t75;
        let t1125 = 2.0 * t1021 * t4 * t79 * t82 - 28.0 * t186 * t48 * t602 * t75 + 6.0 * t1105 * t439 * t78 - 6.0 * t192 * t453 * t82 + 8.0 * t198 * t261 * t453 + 6.0 * t1044 * t205 - 12.0 * t1065 * t205 - 12.0 * t1068 * t205 - 2.0 * t1082 * t205 - t1082 * t78 - 12.0 * t1084 * t205 - 6.0 * t1084 * t78 - 8.0 * t1090 * t197 - 3.0 * t187 * t430 - 3.0 * t192 * t458 - 6.0 * t430 * t472 + 12.0 * t462 * t472 + 12.0 * t465 * t472;
        let t1130 = piecewise3(t31, 0.0, t1004 * t43 / 2.0 + 2.0 * t419 * t87 + 3.0 * t176 * t211 + 2.0 * t71 * t483 + t33 * (t1077 + t1125) / 2.0);
        let tv4rho40 = 4.0 * t417 + 4.0 * t487 + t3 * (t1002 + t1130);
        v4rho4[ip * 5] += tv4rho40;
        let t1135 = 12.0 * t348;
        let t1136 = 24.0 * t876;
        let t1137 = -t1135 + t1136;
        let t1138 = piecewise5(t7, 0.0, t10, 0.0, t1137);
        let t1153 = t525 * t153;
        let t1155 = piecewise5(t16, 0.0, t19, 0.0, t1137);
        let t1158 = t1155 * t3 + t380 + 3.0 * t517;
        let t1161 = t226 * t365;
        let t1164 = 6.0 * t100 * t898;
        let t1169 = t226 * t395;
        let t1171 = t100 * t916;
        let t1173 = t511 * t540;
        let t1175 = t377 * t225;
        let t1176 = t230 * t1175;
        let t1179 = t60 * t521 * t48;
        let t1183 = t100 * t940;
        let t1186 = t230 * t905 * t99;
        let t1189 = t60 * t234 * t134;
        let t1193 = 12.0 * t60 * t102 * t348;
        let t1199 = t1158 * t58 * param_B - 6.0 * t100 * t937 - t100 * t954 - 3.0 * t151 * t525 - 3.0 * t226 * t363 + 6.0 * t226 * t392 + 12.0 * t238 * t909 - 3.0 * t1153 + 6.0 * t1161 - t1164 + 6.0 * t1169 + 3.0 * t1171 - 10.0 * t1173 - 18.0 * t1176 - 6.0 * t1179 - 6.0 * t1183 + 22.0 * t1186 + 12.0 * t1189 - t1193;
        let t1200 = t100 * t956;
        let t1207 = t157 * t520;
        let t1216 = t238 * t940;
        let t1218 = t238 * t956;
        let t1222 = t511 * t372;
        let t1225 = t48 * t225;
        let t1226 = t1225 * t53;
        let t1229 = t60 * t520;
        let t1234 = t543 * t395;
        let t1236 = t238 * t916;
        let t1245 = t230 * t923;
        let t1249 = t134 * t99;
        let t1251 = t535 * t1249 * t53;
        let t1253 = -6.0 * t1200 + 2.0 * param_C * t383 * t231 + 6.0 * t511 * t514 + 6.0 * t230 * t1207 + 2.0 * t60 * t1158 * t57 * t4 - 12.0 * t238 * t937 - 12.0 * t1216 - 12.0 * t1218 + 6.0 * t680 * t977 - 10.0 * t1222 * t537 - 18.0 * t535 * t1226 - 6.0 * t1229 * t151 + 12.0 * t543 * t392 + 12.0 * t1234 + 6.0 * t1236 - 8.0 * t535 * t536 * t142 - 6.0 * t543 * t363 - 2.0 * t238 * t954 + 22.0 * t1245 * t536 * t391 + 28.0 * t1251;
        let t1258 = piecewise3(t1, 0.0, t1138 * t27 / 2.0 + 3.0 / 2.0 * t495 * t65 + 3.0 / 2.0 * t219 * t170 + t95 * t413 / 2.0 + t352 * t106 / 2.0 + 3.0 / 2.0 * t138 * t244 + 3.0 / 2.0 * t51 * t554 + t13 * (t1199 + t1253) / 2.0);
        let t1260 = piecewise5(t10, 0.0, t7, 0.0, -t1137);
        let t1275 = t592 * t194;
        let t1277 = 24.0 * t1015;
        let t1279 = piecewise5(t19, 0.0, t16, 0.0, t1135 + t1277);
        let t1282 = t1279 * t3 + t450 + 3.0 * t584;
        let t1285 = t257 * t432;
        let t1288 = 6.0 * t118 * t1026;
        let t1293 = t257 * t465;
        let t1295 = t118 * t1044;
        let t1297 = t576 * t607;
        let t1299 = t444 * t256;
        let t1300 = t261 * t1299;
        let t1303 = t82 * t588 * t48;
        let t1307 = t118 * t1068;
        let t1310 = t261 * t1033 * t117;
        let t1313 = t82 * t265 * t134;
        let t1317 = 12.0 * t82 * t120 * t348;
        let t1323 = t1282 * t80 * param_B + 12.0 * t1037 * t269 - 6.0 * t1065 * t118 - t1082 * t118 - 3.0 * t192 * t592 - 3.0 * t257 * t430 + 6.0 * t257 * t462 - 3.0 * t1275 + 6.0 * t1285 - t1288 + 6.0 * t1293 + 3.0 * t1295 - 10.0 * t1297 - 18.0 * t1300 - 6.0 * t1303 - 6.0 * t1307 + 22.0 * t1310 + 12.0 * t1313 - t1317;
        let t1324 = t118 * t1084;
        let t1331 = t198 * t587;
        let t1340 = t269 * t1068;
        let t1342 = t269 * t1084;
        let t1346 = t576 * t439;
        let t1349 = t48 * t256;
        let t1350 = t1349 * t75;
        let t1353 = t82 * t587;
        let t1358 = t610 * t465;
        let t1360 = t269 * t1044;
        let t1369 = t261 * t1051;
        let t1373 = t134 * t117;
        let t1375 = t602 * t1373 * t75;
        let t1377 = -6.0 * t1324 + 2.0 * param_C * t453 * t262 + 6.0 * t576 * t579 + 6.0 * t261 * t1331 + 2.0 * t82 * t1282 * t79 * t4 - 12.0 * t269 * t1065 - 12.0 * t1340 - 12.0 * t1342 + 6.0 * t751 * t1105 - 10.0 * t1346 * t604 - 18.0 * t602 * t1350 - 6.0 * t1353 * t192 + 12.0 * t610 * t462 + 12.0 * t1358 + 6.0 * t1360 - 8.0 * t602 * t603 * t183 - 6.0 * t610 * t430 - 2.0 * t269 * t1082 + 22.0 * t1369 * t603 * t461 + 28.0 * t1375;
        let t1382 = piecewise3(t31, 0.0, t1260 * t43 / 2.0 + 3.0 / 2.0 * t560 * t87 + 3.0 / 2.0 * t249 * t211 + t112 * t483 / 2.0 + t419 * t124 / 2.0 + 3.0 / 2.0 * t176 * t275 + 3.0 / 2.0 * t71 * t621 + t33 * (t1323 + t1377) / 2.0);
        let tv4rho41 = t417 + t487 + 3.0 * t558 + 3.0 * t625 + t3 * (t1258 + t1382);
        v4rho4[ip * 5 + 1] += tv4rho41;
        let t1389 = piecewise5(t7, 0.0, t10, 0.0, t1136);
        let t1404 = piecewise5(t16, 0.0, t19, 0.0, t1136);
        let t1408 = t1404 * t3 + 2.0 * t517 + 2.0 * t649;
        let t1416 = t535 * t1249 * t97;
        let t1418 = t377 * t97;
        let t1419 = t238 * t1418;
        let t1421 = t100 * t923;
        let t1426 = t48 * t97 * t53;
        let t1428 = 4.0 * t680 * t1426;
        let t1432 = 4.0 * t685 * t395;
        let t1440 = t644 * t372;
        let t1443 = t60 * t651;
        let t1449 = -4.0 * t299 * t962 + 2.0 * t60 * t1408 * t57 * t4 + 2.0 * t511 * t656 + 6.0 * t1416 - 4.0 * t1419 - 6.0 * t1421 * t681 * t391 - t1428 + 4.0 * t685 * t392 + t1432 + 4.0 * t226 * t372 * t682 + 4.0 * t680 * t4 * t223 * t53 - 16.0 * t1440 * t1226 - 4.0 * t1443 * t151 - 2.0 * t1222 * t675 - t1153 + 4.0 * t1161 - t1164;
        let t1451 = 2.0 * t663 * t153;
        let t1454 = t294 * t365;
        let t1456 = t225 * t225;
        let t1460 = t299 * t905;
        let t1481 = 8.0 * t238 * t373 * t665;
        let t1482 = t1225 * t97;
        let t1488 = t48 * t293;
        let t1489 = t1488 * t53;
        let t1493 = 4.0 * t543 * t669;
        let t1494 = t157 * t223;
        let t1496 = 4.0 * t238 * t1494;
        let t1502 = -t1451 + param_B * t1408 * t58 + 2.0 * t1454 + 4.0 * param_C * t1456 * t157 + 12.0 * t1460 + 10.0 * t1245 * t536 * t665 + 8.0 * t543 * t666 + 8.0 * t238 * t390 * t223 * t53 + 4.0 * t238 * t390 * t97 * t142 - 12.0 * t238 * t935 * t97 * t391 - t1481 - 4.0 * t535 * t1482 - 4.0 * t535 * t536 * t223 - 6.0 * t535 * t1489 + t1493 + t1496 + 2.0 * t680 * t681 * t142 - 2.0 * t685 * t363;
        let t1508 = t150 * t517;
        let t1519 = t157 * t651;
        let t1525 = 2.0 * t294 * t395;
        let t1527 = 2.0 * t226 * t669;
        let t1529 = 2.0 * t100 * t1494;
        let t1530 = -2.0 * t1229 * t296 - 2.0 * t1508 * t238 + 4.0 * t1519 * t230 + 2.0 * t294 * t392 - 4.0 * t543 * t642 + 2.0 * t1169 + t1171 - 2.0 * t1173 - 4.0 * t1176 - 2.0 * t1179 - 2.0 * t1183 + 10.0 * t1186 + 8.0 * t1189 - t1193 - 4.0 * t1200 + t1525 + t1527 + t1529;
        let t1531 = t644 * t1175;
        let t1535 = t299 * t929;
        let t1539 = 4.0 * t60 * t652 * t48;
        let t1540 = t377 * t293;
        let t1541 = t230 * t1540;
        let t1543 = t100 * t1418;
        let t1546 = t60 * t302 * t134;
        let t1562 = -t100 * t1508 + 4.0 * t1207 * t644 - 2.0 * t151 * t663 - 2.0 * t226 * t642 - t294 * t363 - t296 * t525 + 12.0 * t299 * t925 - 4.0 * t1216 - 8.0 * t1218 + 4.0 * t1234 + 2.0 * t1236 + 6.0 * t1251 - 16.0 * t1531 + 16.0 * t1535 - t1539 - 6.0 * t1541 - 2.0 * t1543 + 4.0 * t1546;
        let t1568 = piecewise3(t1, 0.0, t1389 * t27 / 2.0 + t629 * t65 + t286 * t170 / 2.0 + t495 * t106 + 2.0 * t219 * t244 + t95 * t554 + t138 * t308 / 2.0 + t51 * t693 + t13 * (t1449 + t1502 + t1530 + t1562) / 2.0);
        let t1569 = piecewise5(t10, 0.0, t7, 0.0, -t1136);
        let t1582 = t198 * t716;
        let t1588 = 2.0 * t324 * t465;
        let t1590 = 2.0 * t257 * t740;
        let t1591 = t198 * t254;
        let t1593 = 2.0 * t118 * t1591;
        let t1594 = t728 * t1299;
        let t1598 = t329 * t1057;
        let t1602 = 4.0 * t82 * t733 * t48;
        let t1603 = t444 * t323;
        let t1604 = t261 * t1603;
        let t1606 = t444 * t115;
        let t1607 = t118 * t1606;
        let t1610 = t82 * t332 * t134;
        let t1618 = t191 * t584;
        let t1620 = 12.0 * t1053 * t329 - t118 * t1618 + 4.0 * t1582 * t261 - 2.0 * t192 * t717 - 2.0 * t257 * t725 - t324 * t430 + 2.0 * t324 * t462 - t326 * t592 + t1588 + t1590 + t1593 - 16.0 * t1594 + 16.0 * t1598 - t1602 - 6.0 * t1604 - 2.0 * t1607 + 4.0 * t1610;
        let t1625 = piecewise5(t19, 0.0, t16, 0.0, t1277);
        let t1629 = t1625 * t3 + 2.0 * t584 + 2.0 * t714;
        let t1638 = 2.0 * t717 * t194;
        let t1641 = t324 * t432;
        let t1643 = t256 * t256;
        let t1647 = t329 * t1033;
        let t1668 = 8.0 * t269 * t440 * t719;
        let t1669 = 4.0 * t728 * t1331 - 4.0 * t329 * t1090 + 2.0 * t82 * t1629 * t79 * t4 + 2.0 * t576 * t737 - t1275 + 4.0 * t1285 - t1288 - t1638 + param_B * t1629 * t80 + 2.0 * t1641 + 4.0 * param_C * t1643 * t198 + 12.0 * t1647 + 10.0 * t1369 * t603 * t719 + 8.0 * t610 * t720 + 8.0 * t269 * t460 * t254 * t75 + 4.0 * t269 * t460 * t115 * t183 - 12.0 * t269 * t1063 * t115 * t461 - t1668;
        let t1672 = t602 * t1373 * t115;
        let t1674 = t269 * t1606;
        let t1676 = t118 * t1051;
        let t1681 = t48 * t115 * t75;
        let t1683 = 4.0 * t751 * t1681;
        let t1687 = 4.0 * t756 * t465;
        let t1695 = t728 * t439;
        let t1698 = t82 * t716;
        let t1703 = t1349 * t115;
        let t1709 = t48 * t323;
        let t1710 = t1709 * t75;
        let t1714 = 4.0 * t610 * t740;
        let t1716 = 4.0 * t269 * t1591;
        let t1722 = 4.0 * t254 * t4 * t75 * t751 - 6.0 * t1676 * t461 * t752 + 2.0 * t183 * t751 * t752 - 4.0 * t254 * t602 * t603 + 4.0 * t257 * t439 * t753 - 2.0 * t1346 * t746 - 16.0 * t1350 * t1695 - 4.0 * t1698 * t192 - 4.0 * t1703 * t602 - 6.0 * t1710 * t602 - 2.0 * t430 * t756 + 4.0 * t462 * t756 + 6.0 * t1672 - 4.0 * t1674 - t1683 + t1687 + t1714 + t1716;
        let t1742 = -2.0 * t1353 * t326 - 2.0 * t1618 * t269 - 4.0 * t610 * t725 + 2.0 * t1293 + t1295 - 2.0 * t1297 - 4.0 * t1300 - 2.0 * t1303 - 2.0 * t1307 + 10.0 * t1310 + 8.0 * t1313 - t1317 - 4.0 * t1324 - 4.0 * t1340 - 8.0 * t1342 + 4.0 * t1358 + 2.0 * t1360 + 6.0 * t1375;
        let t1748 = piecewise3(t31, 0.0, t1569 * t43 / 2.0 + t699 * t87 + t314 * t211 / 2.0 + t560 * t124 + 2.0 * t249 * t275 + t112 * t621 + t176 * t338 / 2.0 + t71 * t764 + t33 * (t1620 + t1669 + t1722 + t1742) / 2.0);
        let tv4rho42 = 2.0 * t558 + 2.0 * t625 + 2.0 * t697 + 2.0 * t768 + t3 * (t1568 + t1748);
        v4rho4[ip * 5 + 2] += tv4rho42;
        let t1753 = t1135 + t1136;
        let t1754 = piecewise5(t7, 0.0, t10, 0.0, t1753);
        let t1769 = t294 * t669;
        let t1771 = t157 * t290;
        let t1772 = t100 * t1771;
        let t1773 = t644 * t1540;
        let t1776 = t299 * t928 * t97;
        let t1779 = t60 * t797 * t48;
        let t1781 = t373 * t804;
        let t1782 = t100 * t1781;
        let t1789 = t150 * t649;
        let t1799 = piecewise5(t16, 0.0, t19, 0.0, t1753);
        let t1802 = t1799 * t3 + 3.0 * t649 + t793;
        let t1810 = t157 * t796;
        let t1813 = 2.0 * t1769 + t1772 - 12.0 * t1773 + 12.0 * t1776 - 2.0 * t1779 - 2.0 * t1782 - 2.0 * t663 * t296 - 2.0 * t294 * t642 - t226 * t786 - t100 * t1789 + 6.0 * param_C * t225 * t656 + 6.0 * t644 * t1519 - 6.0 * t299 * t373 * t223 + 2.0 * t60 * t1802 * t57 * t4 - t801 * t151 + 2.0 * t226 * t805 + 2.0 * t230 * t1810;
        let t1821 = t801 * t153;
        let t1836 = 8.0 * t223 * t238 * t390 * t97 + 4.0 * t238 * t290 * t390 * t53 - 12.0 * t238 * t53 * t804 * t935 + t1802 * t58 * param_B + 8.0 * t666 * t685 + 2.0 * t1161 - t1164 + 4.0 * t1416 - 8.0 * t1419 - t1428 + t1432 - t1451 + 4.0 * t1454 + 18.0 * t1460 - t1481 + t1493 - t1821;
        let t1838 = t238 * t1781;
        let t1840 = t685 * t669;
        let t1842 = t238 * t1771;
        let t1847 = t4 * t290;
        let t1858 = t60 * t796;
        let t1878 = t1496 - 4.0 * t1838 + 4.0 * t1840 + 2.0 * t1842 + 4.0 * t294 * t372 * t682 + 2.0 * t680 * t1847 * t53 - 12.0 * t1440 * t1489 - 12.0 * t1440 * t1482 + 18.0 * t299 * t923 * t1426 - 2.0 * t1858 * t151 + 4.0 * t543 * t805 - 6.0 * t1421 * t4 * t804 * t53 + 4.0 * t680 * t681 * t223 - 4.0 * t1443 * t296 - 4.0 * t685 * t642 - 2.0 * t543 * t786 - 2.0 * t238 * t1789;
        let t1882 = t1488 * t97;
        let t1897 = 4.0 * t1245 * t536 * t804 - 2.0 * t290 * t535 * t536 - 4.0 * t1882 * t535 + 4.0 * t1186 + 4.0 * t1189 - t1193 - 2.0 * t1200 - 4.0 * t1218 + t1525 + t1527 + t1529 - 12.0 * t1531 + 12.0 * t1535 - t1539 - 4.0 * t1541 - 4.0 * t1543 + 8.0 * t1546;
        let t1903 = piecewise3(t1, 0.0, t1754 * t27 / 2.0 + t775 * t65 / 2.0 + 3.0 / 2.0 * t629 * t106 + 3.0 / 2.0 * t286 * t244 + 3.0 / 2.0 * t219 * t308 + 3.0 / 2.0 * t95 * t693 + t51 * t816 / 2.0 + t13 * (t1813 + t1836 + t1878 + t1897) / 2.0);
        let t1905 = piecewise5(t10, 0.0, t7, 0.0, -t1753);
        let t1925 = t191 * t714;
        let t1939 = piecewise5(t19, 0.0, t16, 0.0, -t1135 + t1277);
        let t1942 = t1939 * t3 + 3.0 * t714 + t839;
        let t1947 = t198 * t842;
        let t1950 = t324 * t740;
        let t1952 = t198 * t320;
        let t1953 = t118 * t1952;
        let t1954 = t728 * t1603;
        let t1956 = t440 * t846;
        let t1957 = t118 * t1956;
        let t1960 = t329 * t1056 * t115;
        let t1963 = t82 * t861 * t48;
        let t1965 = -2.0 * t717 * t326 - 2.0 * t324 * t725 - t257 * t833 - t118 * t1925 + 6.0 * param_C * t256 * t737 + 6.0 * t728 * t1582 - t843 * t192 + 2.0 * t257 * t847 - 6.0 * t329 * t440 * t254 + 2.0 * t82 * t1942 * t79 * t4 + 2.0 * t261 * t1947 + 2.0 * t1950 + t1953 - 12.0 * t1954 - 2.0 * t1957 + 12.0 * t1960 - 2.0 * t1963;
        let t1976 = t843 * t194;
        let t1981 = -12.0 * t1063 * t269 * t75 * t846 + t1942 * t80 * param_B + 2.0 * t1285 - t1288 + t1588 + t1590 + t1593 - 12.0 * t1594 + 12.0 * t1598 - t1602 - 4.0 * t1604 - 4.0 * t1607 + 8.0 * t1610 - t1638 + 4.0 * t1641 + 18.0 * t1647 - t1976;
        let t1995 = t269 * t1956;
        let t1997 = t756 * t740;
        let t1999 = t269 * t1952;
        let t2004 = t4 * t320;
        let t2012 = 8.0 * t115 * t254 * t269 * t460 + 4.0 * t269 * t320 * t460 * t75 + 2.0 * t2004 * t75 * t751 + 4.0 * t324 * t439 * t753 - 12.0 * t1695 * t1710 + 4.0 * t610 * t847 + 8.0 * t720 * t756 - t1668 + 4.0 * t1672 - 8.0 * t1674 - t1683 + t1687 + t1714 + t1716 - 4.0 * t1995 + 4.0 * t1997 + 2.0 * t1999;
        let t2033 = t82 * t842;
        let t2039 = t1709 * t115;
        let t2049 = -6.0 * t1676 * t4 * t846 * t75 + 4.0 * t751 * t752 * t254 - 4.0 * t1698 * t326 - 4.0 * t756 * t725 - 2.0 * t610 * t833 - 2.0 * t269 * t1925 - 12.0 * t1695 * t1703 + 18.0 * t329 * t1051 * t1681 - 2.0 * t2033 * t192 + 4.0 * t1369 * t603 * t846 - 4.0 * t602 * t2039 - 2.0 * t602 * t603 * t320 + 4.0 * t1310 + 4.0 * t1313 - t1317 - 2.0 * t1324 - 4.0 * t1342;
        let t2055 = piecewise3(t31, 0.0, t1905 * t43 / 2.0 + t822 * t87 / 2.0 + 3.0 / 2.0 * t699 * t124 + 3.0 / 2.0 * t314 * t275 + 3.0 / 2.0 * t249 * t338 + 3.0 / 2.0 * t112 * t764 + t71 * t865 / 2.0 + t33 * (t1965 + t1981 + t2012 + t2049) / 2.0);
        let tv4rho43 = 3.0 * t697 + 3.0 * t768 + t820 + t869 + t3 * (t1903 + t2055);
        v4rho4[ip * 5 + 3] += tv4rho43;
        let t2061 = 24.0 * t348 + 24.0 * t876;
        let t2062 = piecewise5(t7, 0.0, t10, 0.0, t2061);
        let t2075 = t150 * t793;
        let t2082 = piecewise5(t16, 0.0, t19, 0.0, t2061);
        let t2085 = t2082 * t3 + 4.0 * t793;
        let t2096 = t935 * t804 * t97;
        let t2107 = 2.0 * t2085 * t4 * t57 * t60 - 8.0 * t290 * t299 * t373 + 22.0 * t299 * t804 * t924 - t100 * t2075 - 6.0 * t100 * t2096 + 8.0 * t1810 * t644 - 3.0 * t294 * t786 + 6.0 * t294 * t805 - 3.0 * t296 * t801 - t1164 - 12.0 * t1419 + 6.0 * t1454 + 6.0 * t1769 + 3.0 * t1772 - 28.0 * t1773 + 28.0 * t1776 - 6.0 * t1779 - 6.0 * t1782;
        let t2112 = t293 * t293;
        let t2140 = 12.0 * t238 * t290 * t390 * t97 + 6.0 * t157 * t2112 * param_C + 6.0 * t1847 * t680 * t97 + t2085 * t58 * param_B - 28.0 * t1440 * t1882 - 6.0 * t1858 * t296 - 2.0 * t2075 * t238 - 12.0 * t2096 * t238 - 6.0 * t685 * t786 + 12.0 * t685 * t805 - t1193 + 22.0 * t1460 - 6.0 * t1543 + 12.0 * t1546 - 3.0 * t1821 - 12.0 * t1838 + 12.0 * t1840 + 6.0 * t1842;
        let t2145 = piecewise3(t1, 0.0, t2062 * t27 / 2.0 + 2.0 * t775 * t106 + 3.0 * t286 * t308 + 2.0 * t95 * t816 + t13 * (t2107 + t2140) / 2.0);
        let t2147 = piecewise5(t10, 0.0, t7, 0.0, -t2061);
        let t2158 = t191 * t839;
        let t2164 = piecewise5(t19, 0.0, t16, 0.0, -24.0 * t348 + 24.0 * t1015);
        let t2167 = t2164 * t3 + 4.0 * t839;
        let t2175 = t1063 * t846 * t115;
        let t2194 = 2.0 * t2167 * t4 * t79 * t82 + 22.0 * t1052 * t329 * t846 - 8.0 * t320 * t329 * t440 - t118 * t2158 - 6.0 * t118 * t2175 + 8.0 * t1947 * t728 - 3.0 * t324 * t833 + 6.0 * t324 * t847 - 3.0 * t326 * t843 - t1288 - 6.0 * t1607 + 12.0 * t1610 + 6.0 * t1950 + 3.0 * t1953 - 28.0 * t1954 - 6.0 * t1957 + 28.0 * t1960 - 6.0 * t1963;
        let t2200 = t323 * t323;
        let t2227 = 12.0 * t115 * t269 * t320 * t460 + 6.0 * t115 * t2004 * t751 + 6.0 * t198 * t2200 * param_C + t2167 * t80 * param_B - 28.0 * t1695 * t2039 - 6.0 * t2033 * t326 - 2.0 * t2158 * t269 - 12.0 * t2175 * t269 - 6.0 * t756 * t833 + 12.0 * t756 * t847 - t1317 + 6.0 * t1641 + 22.0 * t1647 - 12.0 * t1674 - 3.0 * t1976 - 12.0 * t1995 + 12.0 * t1997 + 6.0 * t1999;
        let t2232 = piecewise3(t31, 0.0, t2147 * t43 / 2.0 + 2.0 * t822 * t124 + 3.0 * t314 * t338 + 2.0 * t112 * t865 + t33 * (t2194 + t2227) / 2.0);
        let tv4rho44 = 4.0 * t820 + 4.0 * t869 + t3 * (t2145 + t2232);
        v4rho4[ip * 5 + 4] += tv4rho44;
    }
}
