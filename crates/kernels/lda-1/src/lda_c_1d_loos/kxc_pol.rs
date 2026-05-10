//! LDA_C_1D_LOOS kxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 22 shared lines across all orders.
//! Delta: 27 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};

/// LDA_C_1D_LOOS kxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_1d_loos_kxc_pol(
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
        // --- shared preamble (22 lines) ---
        let t1 = rho0 + rho1;
        let t2 = 1.0 / t1;
        let t4 = 1.0 + 0.6166 * t2;
        let t5 = f64::sqrt(t4);
        let t6 = t5 - 1.0;
        let t7 = t6 * t6;
        let t8 = t1 * t1;
        let t9 = t7 * t8;
        let t10 = M_SQRT2;
        let t11 = f64::sqrt(M_PI);
        let t13 = f64::ln(t10 * t11);
        let t15 = -0.3083 * t13 - 0.231225;
        let t16 = t6 * t1;
        let t18 = 1.0 - 3.243593902043464 * t16;
        let t19 = t18 * t18;
        let t23 = -1.2332 * t13 - 0.8632856383593266;
        let t24 = t23 * t6;
        let t30 = t7 * t6;
        let t31 = t8 * t1;
        let t34 = t15 * t19 * t18 + 3.243593902043464 * t24 * t1 * t19 - 1.1985261315879494 * t9 * t18 + 0.2436562958345998 * t30 * t31;
        let t35 = t9 * t34;
        let tzk0 = 10.520901401373546 * t35;
        zk[ip] += tzk0;
        // --- vxc delta (11 lines) ---
        let t37 = 1.0 / t5;
        let t38 = t34 * t37;
        let t41 = t31 * t7;
        let t42 = t15 * t19;
        let t46 = 1.0 * t37 * t2 - 3.243593902043464 * t5 + 3.243593902043464;
        let t49 = t23 * t37;
        let t59 = t6 * t18;
        let t62 = t7 * t1;
        let t71 = 3.0 * t42 * t46 - 1.0 * t49 * t2 * t19 + 3.243593902043464 * t24 * t19 + 6.487187804086928 * t24 * t1 * t18 * t46 + 0.7390112127371297 * t59 * t37 - 2.397052263175899 * t62 * t18 - 1.1985261315879494 * t9 * t46 - 0.22535770801742136 * t62 * t37 + 0.7309688875037994 * t30 * t8;
        let tvrho0 = 31.56270420412064 * t35 - 6.487187804086928 * t16 * t38 + 10.520901401373546 * t41 * t71;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        // --- fxc delta (20 lines) ---
        let t74 = t6 * t34;
        let t81 = 1.0 / t4;
        let t82 = t2 * t81;
        let t85 = t71 * t37;
        let t88 = t2 * t6;
        let t90 = 1.0 / t5 / t4;
        let t94 = t15 * t18;
        let t95 = t46 * t46;
        let t98 = 1.0 / t31;
        let t99 = t90 * t98;
        let t102 = t23 * t90;
        let t109 = 1.0 / t8;
        let t110 = t90 * t109;
        let t116 = t6 * t46;
        let t119 = t81 * t109;
        let t126 = t7 * t2;
        let t146 = 6.0 * t94 * t95 + 0.9249 * t42 * t99 - 0.3083 * t102 * t98 * t19 + 12.974375608173856 * t24 * t18 * t46 + 0.22783715688685707 * t59 * t110 + 1.4780224254742593 * t88 * t18 * t37 + 1.4780224254742593 * t116 * t37 - 0.22783715688685707 * t119 * t18 - 2.397052263175899 * t7 * t18 - 4.794104526351798 * t62 * t46 - 0.43898338775033585 * t126 * t90 - 0.9014308320696854 * t7 * t37 + 0.138955562763542 * t88 * t81 + 1.4619377750075988 * t30 * t1 - 4.0 * t49 * t2 * t18 * t46 + 6.487187804086928 * t24 * t1 * t95 + 2.0 * t24 * t109 * t18 * t90;
        let tv2rho20 = -25.948751216347713 * t74 * t37 + 63.12540840824128 * t62 * t34 + 63.12540840824128 * t9 * t71 + 2.0 * t82 * t34 - 12.974375608173856 * t16 * t85 - 2.0 * t88 * t34 * t90 + 10.520901401373546 * t41 * t146;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        // --- kxc delta (this level) (27 lines) ---
        let t151 = t6 * t71;
        let t164 = t4 * t4;
        let t165 = 1.0 / t164;
        let t166 = t98 * t165;
        let t177 = t98 * t6;
        let t179 = 1.0 / t5 / t164;
        let t190 = t46 * t90;
        let t202 = t8 * t8;
        let t203 = 1.0 / t202;
        let t204 = t203 * t18;
        let t210 = t165 * t203;
        let t213 = t81 * t98;
        let t221 = -7.191156789527697 * t7 * t46 + 6.0 * t15 * t95 * t46 - 0.04284 * t99 + 1.4619377750075988 * t30 + 5.5494 * t94 * t190 * t98 - 1.8498 * t102 * t98 * t18 * t46 + 6.0 * t24 * t109 * t46 * t90 + 1.8498 * t24 * t204 * t179 + 19.461563412260784 * t24 * t95 - 0.2107265864046541 * t210 * t18 - 2e-20 * t213 * t18 - 0.6835114706605712 * t119 * t46 + 0.8120314706605712 * t6 * t165 * t98;
        let t228 = t6 * t81;
        let t234 = 1.0 / t202 / t1;
        let t235 = t179 * t234;
        let t238 = t23 * t179;
        let t239 = t234 * t19;
        let t244 = t179 * t203;
        let t250 = t23 * t165;
        let t253 = t90 * t203;
        let t264 = -1.3169501632510074 * t7 * t109 * t90 - 0.4060157353302856 * t7 * t98 * t179 + 0.416866688290626 * t228 * t109 - 1.352146248104528 * t126 * t37 + 0.85544001 * t42 * t235 - 0.28514667 * t238 * t239 + 0.6835114706605712 * t116 * t110 + 0.2107265864046541 * t59 * t244 + 4.434067276422778 * t88 * t46 * t37 - 1.8498 * t250 * t204 - 2.7747 * t42 * t253 + 0.9249 * t102 * t203 * t19 + 2e-20 * t59 * t99 - 6.0 * t49 * t2 * t95;
        let t265 = t221 + t264;
        let tv3rho30 = 6.0 * t119 * t34 - 77.84625364904313 * t151 * t37 - 6.0 * t74 * t110 - 38.92312682452157 * t88 * t38 + 63.12540840824128 * t7 * t34 + 189.3762252247238 * t62 * t71 + 94.6881126123619 * t9 * t146 + 1.8498 * t166 * t34 + 6.0 * t82 * t71 - 19.461563412260784 * t16 * t146 * t37 - 6.0 * t88 * t71 * t90 - 1.8498 * t177 * t34 * t179 + 10.520901401373546 * t41 * t265;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
