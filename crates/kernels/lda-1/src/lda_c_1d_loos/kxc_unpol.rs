//! LDA_C_1D_LOOS kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 21 shared lines across all orders.
//! Delta: 25 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};

/// LDA_C_1D_LOOS kxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_1d_loos_kxc_unpol(
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
        // --- shared preamble (21 lines) ---
        let t1 = 1.0 / rho[ip];
        let t3 = 1.0 + 0.6166 * t1;
        let t4 = f64::sqrt(t3);
        let t5 = t4 - 1.0;
        let t6 = t5 * t5;
        let t7 = rho[ip] * rho[ip];
        let t8 = t6 * t7;
        let t9 = M_SQRT2;
        let t10 = f64::sqrt(M_PI);
        let t12 = f64::ln(t9 * t10);
        let t14 = -0.3083 * t12 - 0.231225;
        let t15 = t5 * rho[ip];
        let t17 = 1.0 - 3.243593902043464 * t15;
        let t18 = t17 * t17;
        let t22 = -1.2332 * t12 - 0.8632856383593266;
        let t23 = t22 * t5;
        let t29 = t6 * t5;
        let t30 = t7 * rho[ip];
        let t33 = t14 * t18 * t17 + 3.243593902043464 * t23 * rho[ip] * t18 - 1.1985261315879494 * t8 * t17 + 0.2436562958345998 * t29 * t30;
        let t34 = t8 * t33;
        let tzk0 = 10.520901401373546 * t34;
        zk[ip] += tzk0;
        // --- vxc delta (10 lines) ---
        let t36 = 1.0 / t4;
        let t37 = t33 * t36;
        let t40 = t30 * t6;
        let t41 = t14 * t18;
        let t45 = 1.0 * t36 * t1 - 3.243593902043464 * t4 + 3.243593902043464;
        let t48 = t22 * t36;
        let t58 = t5 * t17;
        let t61 = t6 * rho[ip];
        let t70 = 3.0 * t41 * t45 - 1.0 * t48 * t1 * t18 + 3.243593902043464 * t23 * t18 + 6.487187804086928 * t23 * rho[ip] * t17 * t45 + 0.7390112127371297 * t58 * t36 - 2.397052263175899 * t61 * t17 - 1.1985261315879494 * t8 * t45 - 0.22535770801742136 * t61 * t36 + 0.7309688875037994 * t29 * t7;
        let tvrho0 = 31.56270420412064 * t34 - 6.487187804086928 * t15 * t37 + 10.520901401373546 * t40 * t70;
        vrho[ip] += tvrho0;
        // --- fxc delta (18 lines) ---
        let t73 = t5 * t33;
        let t80 = 1.0 / t3;
        let t81 = t1 * t80;
        let t84 = t70 * t36;
        let t87 = t1 * t5;
        let t89 = 1.0 / t4 / t3;
        let t97 = t45 * t45;
        let t101 = 1.0 / t7;
        let t106 = t14 * t17;
        let t109 = 1.0 / t30;
        let t110 = t89 * t109;
        let t113 = t22 * t89;
        let t120 = t89 * t101;
        let t126 = t5 * t45;
        let t129 = t80 * t101;
        let t136 = t6 * t1;
        let t145 = -4.0 * t48 * t1 * t17 * t45 + 6.487187804086928 * t23 * rho[ip] * t97 + 2.0 * t23 * t101 * t17 * t89 + 6.0 * t106 * t97 + 0.9249 * t41 * t110 - 0.3083 * t113 * t109 * t18 + 12.974375608173856 * t23 * t17 * t45 + 0.22783715688685707 * t58 * t120 + 1.4780224254742593 * t87 * t17 * t36 + 1.4780224254742593 * t126 * t36 - 0.22783715688685707 * t129 * t17 - 2.397052263175899 * t6 * t17 - 4.794104526351798 * t61 * t45 - 0.43898338775033585 * t136 * t89 - 0.9014308320696854 * t6 * t36 + 0.138955562763542 * t87 * t80 + 1.4619377750075988 * t29 * rho[ip];
        let tv2rho20 = -25.948751216347713 * t73 * t36 + 63.12540840824128 * t61 * t33 + 63.12540840824128 * t8 * t70 + 2.0 * t81 * t33 - 12.974375608173856 * t15 * t84 - 2.0 * t87 * t33 * t89 + 10.520901401373546 * t40 * t145;
        v2rho2[ip] += tv2rho20;
        // --- kxc delta (this level) (25 lines) ---
        let t150 = t5 * t70;
        let t163 = t3 * t3;
        let t164 = 1.0 / t163;
        let t165 = t109 * t164;
        let t176 = t109 * t5;
        let t178 = 1.0 / t4 / t163;
        let t196 = t7 * t7;
        let t197 = 1.0 / t196;
        let t198 = t197 * t17;
        let t202 = t45 * t89;
        let t210 = t89 * t197;
        let t220 = -7.191156789527697 * t6 * t45 - 0.04284 * t110 + 6.0 * t14 * t97 * t45 - 1.8498 * t113 * t109 * t17 * t45 + 6.0 * t23 * t101 * t45 * t89 + 1.8498 * t23 * t198 * t178 + 5.5494 * t106 * t202 * t109 + 1.4619377750075988 * t29 - 6.0 * t48 * t1 * t97 - 2.7747 * t41 * t210 + 0.9249 * t113 * t197 * t18 + 2e-20 * t58 * t110 + 19.461563412260784 * t23 * t97;
        let t221 = t164 * t197;
        let t224 = t80 * t109;
        let t235 = t6 * t109;
        let t238 = t5 * t80;
        let t243 = t22 * t164;
        let t247 = 1.0 / t196 / rho[ip];
        let t248 = t178 * t247;
        let t251 = t22 * t178;
        let t252 = t247 * t18;
        let t257 = t178 * t197;
        let t263 = -0.2107265864046541 * t221 * t17 - 2e-20 * t224 * t17 - 0.6835114706605712 * t129 * t45 + 0.8120314706605712 * t5 * t164 * t109 - 1.3169501632510074 * t6 * t101 * t89 - 0.4060157353302856 * t235 * t178 + 0.416866688290626 * t238 * t101 - 1.352146248104528 * t136 * t36 - 1.8498 * t243 * t198 + 0.85544001 * t41 * t248 - 0.28514667 * t251 * t252 + 0.6835114706605712 * t126 * t120 + 0.2107265864046541 * t58 * t257 + 4.434067276422778 * t87 * t45 * t36;
        let t264 = t220 + t263;
        let tv3rho30 = 6.0 * t129 * t33 - 77.84625364904313 * t150 * t36 - 6.0 * t73 * t120 - 38.92312682452157 * t87 * t37 + 63.12540840824128 * t6 * t33 + 189.3762252247238 * t61 * t70 + 94.6881126123619 * t8 * t145 + 1.8498 * t165 * t33 + 6.0 * t81 * t70 - 19.461563412260784 * t15 * t145 * t36 - 6.0 * t87 * t70 * t89 - 1.8498 * t176 * t33 * t178 + 10.520901401373546 * t40 * t264;
        v3rho3[ip] += tv3rho30;
    }
}
