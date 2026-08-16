//! LDA_C_1D_LOOS fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_loos.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};

/// LDA_C_1D_LOOS fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_1d_loos_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
    }
}
