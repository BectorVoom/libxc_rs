//! LDA_XC_TETER93 vxc pol kernel (rayon backend).
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
pub fn lda_xc_teter93_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 1.0 + t4;
        let t6 = t5 <= zeta_threshold;
        let t7 = pow_1_3(zeta_threshold);
        let t8 = t7 * zeta_threshold;
        let t9 = pow_1_3(t5);
        let t11 = piecewise3(t6, t8, t9 * t5);
        let t12 = 1.0 - t4;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3(t12);
        let t16 = piecewise3(t13, t8, t14 * t12);
        let t18 = M_CBRT2;
        let t21 = 1.0 / (2.0 * t18 - 2.0);
        let t22 = (t11 + t16 - 2.0) * t21;
        let t26 = M_CBRT3;
        let t27 = (2.217058676663745 + 0.6157402568883344 * t22) * t26;
        let t28 = 1.0 / M_PI;
        let t29 = pow_1_3(t28);
        let t30 = M_CBRT4;
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = pow_1_3(t2);
        let t34 = 1.0 / t33;
        let t35 = t32 * t34;
        let t40 = t26 * t26;
        let t41 = (0.7405551735357053 + 0.1574201515892867 * t22) * t40;
        let t42 = t29 * t29;
        let t43 = t42 * t30;
        let t44 = t33 * t33;
        let t46 = t43 / t44;
        let t51 = (0.01968227878617998 + 0.003532336663397157 * t22) * t28;
        let t54 = 0.4581652932831429 + 0.119086804055547 * t22 + t27 * t35 / 4.0 + t41 * t46 / 4.0 + 3.0 / 4.0 * t51 * t3;
        let t55 = t26 * t29;
        let t61 = (4.504130959426697 + 0.2673612973836267 * t22) * t40;
        let t66 = (1.110667363742916 + 0.2052004607777787 * t22) * t28;
        let t71 = (0.02359291751427506 + 0.004200005045691381 * t22) * t26;
        let t73 = t29 * t28 * t31;
        let t75 = 1.0 / t33 / t2;
        let t76 = t73 * t75;
        let t79 = 0.25 * t55 * t31 * t34 + t61 * t46 / 4.0 + 3.0 / 4.0 * t66 * t3 + 3.0 / 16.0 * t71 * t76;
        let t80 = 1.0 / t79;
        let tzk0 = -t54 * t80;
        zk[ip] += tzk0;
        let t82 = t2 * t2;
        let t83 = 1.0 / t82;
        let t84 = t1 * t83;
        let t85 = t3 - t84;
        let t88 = piecewise3(t6, 0.0, 4.0 / 3.0 * t9 * t85);
        let t89 = -t85;
        let t92 = piecewise3(t13, 0.0, 4.0 / 3.0 * t14 * t89);
        let t94 = (t88 + t92) * t21;
        let t96 = t94 * t26;
        let t99 = t32 * t75;
        let t101 = t27 * t99 / 12.0;
        let t102 = t94 * t40;
        let t103 = t102 * t46;
        let t107 = t43 / t44 / t2;
        let t109 = t41 * t107 / 6.0;
        let t110 = t94 * t3;
        let t113 = 3.0 / 4.0 * t51 * t83;
        let t114 = 0.119086804055547 * t94 + 0.1539350642220836 * t96 * t35 - t101 + 0.03935503789732168 * t103 - t109 + 0.0008432832609665849 * t110 - t113;
        let t115 = t2 * t114;
        let t117 = t2 * t54;
        let t118 = t79 * t79;
        let t119 = 1.0 / t118;
        let t122 = 0.08333333333333333 * t55 * t31 * t75;
        let t125 = t61 * t107 / 6.0;
        let t128 = 3.0 / 4.0 * t66 * t83;
        let t132 = 1.0 / t33 / t82;
        let t133 = t73 * t132;
        let t135 = t71 * t133 / 4.0;
        let t136 = -t122 + 0.06684032434590667 * t103 - t125 + 0.048988001486277105 * t110 - t128 + 0.0007875009460671339 * t96 * t76 - t135;
        let t137 = t119 * t136;
        let tvrho0 = -t115 * t80 + t117 * t137 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t139 = -t3 - t84;
        let t142 = piecewise3(t6, 0.0, 4.0 / 3.0 * t9 * t139);
        let t143 = -t139;
        let t146 = piecewise3(t13, 0.0, 4.0 / 3.0 * t14 * t143);
        let t148 = (t142 + t146) * t21;
        let t150 = t148 * t26;
        let t153 = t148 * t40;
        let t154 = t153 * t46;
        let t156 = t148 * t3;
        let t158 = 0.119086804055547 * t148 + 0.1539350642220836 * t150 * t35 - t101 + 0.03935503789732168 * t154 - t109 + 0.0008432832609665849 * t156 - t113;
        let t159 = t2 * t158;
        let t165 = -t122 + 0.06684032434590667 * t154 - t125 + 0.048988001486277105 * t156 - t128 + 0.0007875009460671339 * t150 * t76 - t135;
        let t166 = t119 * t165;
        let tvrho1 = t117 * t166 - t159 * t80 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
