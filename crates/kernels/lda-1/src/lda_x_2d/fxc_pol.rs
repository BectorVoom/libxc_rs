//! LDA_X_2D fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 24 shared lines across all orders.
//! Delta: 33 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_2D fxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_x_2d_fxc_pol(
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
        // --- shared preamble (24 lines) ---
        let t1 = M_SQRT2;
        let t2 = f64::sqrt(M_PI);
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = rho0 - rho1;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t8 = t5 * t7;
        let t9 = 1.0 + t8;
        let t10 = t9 <= zeta_threshold;
        let t11 = f64::sqrt(zeta_threshold);
        let t12 = t11 * zeta_threshold;
        let t13 = f64::sqrt(t9);
        let t14 = t13 * t9;
        let t15 = piecewise3(t10, t12, t14);
        let t16 = 1.0 - t8;
        let t17 = t16 <= zeta_threshold;
        let t18 = f64::sqrt(t16);
        let t19 = t18 * t16;
        let t20 = piecewise3(t17, t12, t19);
        let t22 = t15 / 2.0 + t20 / 2.0;
        let t23 = f64::sqrt(t6);
        let t25 = t4 * t22 * t23;
        let tzk0 = -4.0 / 3.0 * t25;
        zk[ip] += tzk0;
        // --- vxc delta (18 lines) ---
        let t27 = 2.0 * t25;
        let t28 = t23 * t6;
        let t29 = t28 * t1;
        let t30 = t6 * t6;
        let t31 = 1.0 / t30;
        let t32 = t5 * t31;
        let t33 = t7 - t32;
        let t36 = piecewise3(t10, 0.0, 3.0 / 2.0 * t13 * t33);
        let t37 = -t33;
        let t40 = piecewise3(t17, 0.0, 3.0 / 2.0 * t18 * t37);
        let t42 = t36 / 2.0 + t40 / 2.0;
        let tvrho0 = -t27 - 4.0 / 3.0 * t29 * t3 * t42;
        vrho[ip * 2] += tvrho0;
        let t46 = -t7 - t32;
        let t49 = piecewise3(t10, 0.0, 3.0 / 2.0 * t13 * t46);
        let t50 = -t46;
        let t53 = piecewise3(t17, 0.0, 3.0 / 2.0 * t18 * t50);
        let t56 = t3 * (t49 / 2.0 + t53 / 2.0);
        let tvrho1 = -t27 - 4.0 / 3.0 * t29 * t56;
        vrho[ip * 2 + 1] += tvrho1;
        // --- fxc delta (this level) (33 lines) ---
        let t60 = t4 * t42 * t23;
        let t62 = 1.0 / t23;
        let t64 = t4 * t22 * t62;
        let t65 = 1.0 / t13;
        let t66 = t33 * t33;
        let t70 = 1.0 / t30 / t6;
        let t71 = t5 * t70;
        let t73 = -2.0 * t31 + 2.0 * t71;
        let t77 = piecewise3(t10, 0.0, 3.0 / 4.0 * t65 * t66 + 3.0 / 2.0 * t13 * t73);
        let t78 = 1.0 / t18;
        let t79 = t37 * t37;
        let t82 = -t73;
        let t86 = piecewise3(t17, 0.0, 3.0 / 4.0 * t78 * t79 + 3.0 / 2.0 * t18 * t82);
        let t88 = t77 / 2.0 + t86 / 2.0;
        let tv2rho20 = -4.0 * t60 - t64 - 4.0 / 3.0 * t29 * t3 * t88;
        v2rho2[ip * 3] += tv2rho20;
        let t93 = t23 * t1;
        let t94 = t93 * t56;
        let t96 = t65 * t46;
        let t99 = t13 * t5;
        let t103 = piecewise3(t10, 0.0, 3.0 / 4.0 * t96 * t33 + 3.0 * t99 * t70);
        let t104 = t78 * t50;
        let t107 = t18 * t5;
        let t111 = piecewise3(t17, 0.0, 3.0 / 4.0 * t104 * t37 - 3.0 * t107 * t70);
        let t114 = t3 * (t103 / 2.0 + t111 / 2.0);
        let tv2rho21 = -2.0 * t60 - t64 - 2.0 * t94 - 4.0 / 3.0 * t29 * t114;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t118 = t46 * t46;
        let t122 = 2.0 * t31 + 2.0 * t71;
        let t126 = piecewise3(t10, 0.0, 3.0 / 4.0 * t65 * t118 + 3.0 / 2.0 * t13 * t122);
        let t127 = t50 * t50;
        let t130 = -t122;
        let t134 = piecewise3(t17, 0.0, 3.0 / 4.0 * t78 * t127 + 3.0 / 2.0 * t18 * t130);
        let t137 = t3 * (t126 / 2.0 + t134 / 2.0);
        let tv2rho22 = -4.0 * t94 - t64 - 4.0 / 3.0 * t29 * t137;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
