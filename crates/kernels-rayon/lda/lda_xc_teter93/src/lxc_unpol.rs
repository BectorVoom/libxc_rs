//! LDA_XC_TETER93 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_teter93.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_teter93_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = pow_1_3(zeta_threshold);
        let t4 = piecewise3(1.0 <= zeta_threshold, t2 * zeta_threshold, 1.0);
        let t7 = M_CBRT2;
        let t11 = (2.0 * t4 - 2.0) / (2.0 * t7 - 2.0);
        let t15 = M_CBRT3;
        let t16 = (2.217058676663745 + 0.6157402568883344 * t11) * t15;
        let t17 = 1.0 / M_PI;
        let t18 = pow_1_3(t17);
        let t19 = M_CBRT4;
        let t20 = t19 * t19;
        let t21 = t18 * t20;
        let t22 = pow_1_3(rho[ip]);
        let t23 = 1.0 / t22;
        let t29 = t15 * t15;
        let t30 = (0.7405551735357053 + 0.1574201515892867 * t11) * t29;
        let t31 = t18 * t18;
        let t32 = t31 * t19;
        let t33 = t22 * t22;
        let t35 = t32 / t33;
        let t40 = (0.01968227878617998 + 0.003532336663397157 * t11) * t17;
        let t41 = 1.0 / rho[ip];
        let t44 = 0.4581652932831429 + 0.119086804055547 * t11 + t16 * t21 * t23 / 4.0 + t30 * t35 / 4.0 + 3.0 / 4.0 * t40 * t41;
        let t45 = t15 * t18;
        let t51 = (4.504130959426697 + 0.2673612973836267 * t11) * t29;
        let t56 = (1.110667363742916 + 0.2052004607777787 * t11) * t17;
        let t61 = (0.02359291751427506 + 0.004200005045691381 * t11) * t15;
        let t63 = t18 * t17 * t20;
        let t65 = 1.0 / t22 / rho[ip];
        let t69 = 0.25 * t45 * t20 * t23 + t51 * t35 / 4.0 + 3.0 / 4.0 * t56 * t41 + 3.0 / 16.0 * t61 * t63 * t65;
        let t70 = 1.0 / t69;
        let tzk0 = -t44 * t70;
        zk[ip] += tzk0;
        let t77 = t32 / t33 / rho[ip];
        let t80 = rho[ip] * rho[ip];
        let t81 = 1.0 / t80;
        let t84 = -t16 * t21 * t65 / 12.0 - t30 * t77 / 6.0 - 3.0 / 4.0 * t40 * t81;
        let t85 = rho[ip] * t84;
        let t87 = rho[ip] * t44;
        let t88 = t69 * t69;
        let t89 = 1.0 / t88;
        let t98 = 1.0 / t22 / t80;
        let t102 = -0.08333333333333333 * t45 * t20 * t65 - t51 * t77 / 6.0 - 3.0 / 4.0 * t56 * t81 - t61 * t63 * t98 / 4.0;
        let t103 = t89 * t102;
        let tvrho0 = t87 * t103 - t85 * t70 + tzk0;
        vrho[ip] += tvrho0;
        let t107 = t44 * t89;
        let t115 = t32 / t33 / t80;
        let t118 = t80 * rho[ip];
        let t119 = 1.0 / t118;
        let t122 = t16 * t21 * t98 / 9.0 + 5.0 / 18.0 * t30 * t115 + 3.0 / 2.0 * t40 * t119;
        let t123 = rho[ip] * t122;
        let t128 = 1.0 / t88 / t69;
        let t129 = t102 * t102;
        let t130 = t128 * t129;
        let t141 = 1.0 / t22 / t118;
        let t145 = 0.1111111111111111 * t45 * t20 * t98 + 5.0 / 18.0 * t51 * t115 + 3.0 / 2.0 * t56 * t119 + 7.0 / 12.0 * t61 * t63 * t141;
        let t146 = t89 * t145;
        let tv2rho20 = 2.0 * t107 * t102 + 2.0 * t85 * t103 - t123 * t70 - 2.0 * t87 * t130 + t87 * t146 - 2.0 * t84 * t70;
        v2rho2[ip] += tv2rho20;
        let t150 = t84 * t89;
        let t153 = t44 * t128;
        let t163 = t32 / t33 / t118;
        let t166 = t80 * t80;
        let t167 = 1.0 / t166;
        let t170 = -7.0 / 27.0 * t16 * t21 * t141 - 20.0 / 27.0 * t30 * t163 - 9.0 / 2.0 * t40 * t167;
        let t171 = rho[ip] * t170;
        let t179 = t88 * t88;
        let t180 = 1.0 / t179;
        let t181 = t129 * t102;
        let t182 = t180 * t181;
        let t185 = t128 * t102;
        let t186 = t185 * t145;
        let t197 = 1.0 / t22 / t166;
        let t201 = -0.25925925925925924 * t45 * t20 * t141 - 20.0 / 27.0 * t51 * t163 - 9.0 / 2.0 * t56 * t167 - 35.0 / 18.0 * t61 * t63 * t197;
        let t202 = t89 * t201;
        let tv3rho30 = 6.0 * t150 * t102 + 3.0 * t123 * t103 + 3.0 * t107 * t145 - 3.0 * t122 * t70 - 6.0 * t153 * t129 - 6.0 * t85 * t130 + 3.0 * t85 * t146 - t171 * t70 + 6.0 * t87 * t182 - 6.0 * t87 * t186 + t87 * t202;
        v3rho3[ip] += tv3rho30;
        let t212 = t129 * t129;
        let t220 = t145 * t145;
        let t247 = t32 / t33 / t166;
        let t250 = t166 * rho[ip];
        let t251 = 1.0 / t250;
        let tv4rho40 = -12.0 * t123 * t130 + 24.0 * t85 * t182 - 24.0 * t85 * t186 - 24.0 * t87 / t179 / t69 * t212 + 36.0 * t87 * t180 * t129 * t145 - 6.0 * t87 * t128 * t220 - 8.0 * t87 * t185 * t201 - 24.0 * t84 * t128 * t129 + 24.0 * t44 * t180 * t181 - 24.0 * t153 * t102 * t145 + 4.0 * t171 * t103 + 6.0 * t123 * t146 + 4.0 * t85 * t202 + t87 * t89 * (0.8641975308641975 * t45 * t20 * t197 + 220.0 / 81.0 * t51 * t247 + 18.0 * t56 * t251 + 455.0 / 54.0 * t61 * t63 / t22 / t250) + 12.0 * t122 * t89 * t102 + 12.0 * t150 * t145 + 4.0 * t107 * t201 - rho[ip] * (70.0 / 81.0 * t16 * t21 * t197 + 220.0 / 81.0 * t30 * t247 + 18.0 * t40 * t251) * t70 - 4.0 * t170 * t70;
        v4rho4[ip] += tv4rho40;
    }
}
