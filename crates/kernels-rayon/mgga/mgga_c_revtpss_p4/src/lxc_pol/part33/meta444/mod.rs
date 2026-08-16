//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1618;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1619;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1620;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1621;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1622;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1623;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta444(t20469: f64, t422: f64, t12485: f64, t6518: f64, t5206: f64, t1196: f64, t5192: f64, t5198: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64, t12459: f64, t12460: f64, t16710: f64, t16931: f64, t17066: f64, t17075: f64, t20366: f64, t20368: f64, t20371: f64, t20373: f64, t20378: f64, t12261: f64, t12297: f64, t16706: f64, t16876: f64, t17050: f64, t17052: f64, t20268: f64, t20274: f64, t20276: f64, t20278: f64, t20280: f64, t20338: f64, t20341: f64, t20344: f64, t20347: f64, t20350: f64, t20353: f64, t20357: f64, t20359: f64, t20362: f64, t1169: f64, t1179: f64, t6513: f64, t1188: f64, t20382: f64, t1160: f64, t6481: f64, t1161: f64, t1170: f64, t1180: f64, t1189: f64, t12423: f64, t12481: f64, t12491: f64, t17089: f64, t1757: f64, t20450: f64, t20452: f64, t3491: f64, t5158: f64, t5181: f64, t6506: f64, t6519: f64, t6535: f64, t6538: f64, t12367: f64, t16820: f64, t16821: f64, t16822: f64, t448: f64, t17092: f64, t5068: f64, t16840: f64, t5109: f64, t1149: f64, t6439: f64, t3433: f64, t1733: f64, t5104: f64, t3384: f64, t6474: f64, t12248: f64, t12397: f64, t16708: f64, t17010: f64, t17011: f64, t12511: f64, t17023: f64, t17026: f64, t1745: f64, t3447: f64, t435: f64, t5120: f64, t5125: f64, t5143: f64, t6487: f64, t6503: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20471, t20475, t20477, t20498) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1618(t20469, t422, t12485, t6518, t5206, t1196, t5192, t5198, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t20520 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1619(t12459, t12460, t16710, t16931, t17066, t17075, t20366, t20368, t20371, t20373, t20378, t12261, t12297, t16706, t16876, t17050, t17052, t20268, t20274, t20276, t20278, t20280, t20338, t20341, t20344, t20347, t20350, t20353, t20357, t20359, t20362, t20498);
        let t20545 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1620(t1169, t20520, t1179, t6513, t1188, t20382, t1160, t6481, t1161, t1170, t1180, t1189, t12423, t12481, t12491, t17089, t1757, t20450, t20452, t3491, t5158, t5181, t6506, t6519, t6535, t6538);
        let t20567 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1621(t12297, t12367, t16706, t16820, t16821, t16822, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let (t20568, t20571, t20573, t20576, t20579) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1622(t20567, t448, t17092, t5068, t16840, t5109, t1149, t6439, t3433, t1733, t5104, t3384);
        let (t20582, t20597) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1623(t1149, t6474, t12248, t12297, t12397, t16706, t16708, t17010, t17011, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t20602 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1624(t12511, t17023, t17026, t1745, t20471, t20568, t20571, t20573, t20576, t20579, t20582, t20597, t3447, t435, t5120, t5125, t5143, t6487, t6503);
    (t20471, t20475, t20477, t20545, t20568, t20571, t20573, t20576, t20579, t20582, t20602)
}
