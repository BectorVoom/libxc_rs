//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2580;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2581;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2582;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta643(t1196: f64, t20397: f64, t300: f64, t6513: f64, t1198: f64, t16784: f64, t1765: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64, t12349: f64, t12352: f64, t16708: f64, t16893: f64, t16929: f64, t16931: f64, t20366: f64, t20368: f64, t20371: f64, t20373: f64, t20378: f64, t12261: f64, t12297: f64, t16706: f64, t16869: f64, t16873: f64, t16876: f64, t20268: f64, t20274: f64, t20276: f64, t20278: f64, t20280: f64, t20338: f64, t20341: f64, t20344: f64, t20347: f64, t20350: f64, t20353: f64, t20357: f64, t20359: f64, t20362: f64, t1150: f64, t1131: f64, t12243: f64, t6474: f64, t3531: f64, t6548: f64, t12382: f64, t16797: f64, t16798: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20399, t20400, t20402, t20404, t20425) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2580(t1196, t20397, t300, t6513, t1198, t16784, t1765, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t20447 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2581(t12349, t12352, t16708, t16893, t16929, t16931, t20366, t20368, t20371, t20373, t20378, t12261, t12297, t16706, t16869, t16873, t16876, t20268, t20274, t20276, t20278, t20280, t20338, t20341, t20344, t20347, t20350, t20353, t20357, t20359, t20362, t20425);
        let (t20448, t20450, t20452, t20454, t20469) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2582(t1150, t20447, t1131, t12243, t6474, t3531, t6548, t12297, t12382, t16706, t16708, t16797, t16798, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
    (t20399, t20400, t20402, t20404, t20447, t20448, t20450, t20452, t20454, t20469)
}
