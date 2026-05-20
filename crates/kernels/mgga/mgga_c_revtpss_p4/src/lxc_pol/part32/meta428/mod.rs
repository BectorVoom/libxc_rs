//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1515;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1516;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1517;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1518;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1519;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1520;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta428<F: Float>(t20469: F, t422: F, t12485: F, t6518: F, t5206: F, t1196: F, t5192: F, t5198: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t12459: F, t12460: F, t16710: F, t16931: F, t17066: F, t17075: F, t20366: F, t20368: F, t20371: F, t20373: F, t20378: F, t12261: F, t12297: F, t16706: F, t16876: F, t17050: F, t17052: F, t20268: F, t20274: F, t20276: F, t20278: F, t20280: F, t20338: F, t20341: F, t20344: F, t20347: F, t20350: F, t20353: F, t20357: F, t20359: F, t20362: F, t1169: F, t1179: F, t6513: F, t1188: F, t20382: F, t1160: F, t6481: F, t1161: F, t1170: F, t1180: F, t1189: F, t12423: F, t12481: F, t12491: F, t17089: F, t1757: F, t20450: F, t20452: F, t3491: F, t5158: F, t5181: F, t6506: F, t6519: F, t6535: F, t6538: F, t12367: F, t16820: F, t16821: F, t16822: F, t448: F, t17092: F, t5068: F, t16840: F, t5109: F, t1149: F, t6439: F, t3433: F, t1733: F, t5104: F, t3384: F, t6474: F, t12248: F, t12397: F, t16708: F, t17010: F, t17011: F, t12511: F, t17023: F, t17026: F, t1745: F, t3447: F, t435: F, t5120: F, t5125: F, t5143: F, t6487: F, t6503: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20471, t20475, t20477, t20498) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1515::<F>(t20469, t422, t12485, t6518, t5206, t1196, t5192, t5198, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t20520 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1516::<F>(t12459, t12460, t16710, t16931, t17066, t17075, t20366, t20368, t20371, t20373, t20378, t12261, t12297, t16706, t16876, t17050, t17052, t20268, t20274, t20276, t20278, t20280, t20338, t20341, t20344, t20347, t20350, t20353, t20357, t20359, t20362, t20498);
        let t20545 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1517::<F>(t1169, t20520, t1179, t6513, t1188, t20382, t1160, t6481, t1161, t1170, t1180, t1189, t12423, t12481, t12491, t17089, t1757, t20450, t20452, t3491, t5158, t5181, t6506, t6519, t6535, t6538);
        let t20567 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1518::<F>(t12297, t12367, t16706, t16820, t16821, t16822, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let (t20568, t20571, t20573, t20576, t20579) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1519::<F>(t20567, t448, t17092, t5068, t16840, t5109, t1149, t6439, t3433, t1733, t5104, t3384);
        let (t20582, t20597) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1520::<F>(t1149, t6474, t12248, t12297, t12397, t16706, t16708, t17010, t17011, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t20602 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1521::<F>(t12511, t17023, t17026, t1745, t20471, t20568, t20571, t20573, t20576, t20579, t20582, t20597, t3447, t435, t5120, t5125, t5143, t6487, t6503);
    (t20471, t20475, t20477, t20545, t20568, t20571, t20573, t20576, t20579, t20582, t20602)
}
