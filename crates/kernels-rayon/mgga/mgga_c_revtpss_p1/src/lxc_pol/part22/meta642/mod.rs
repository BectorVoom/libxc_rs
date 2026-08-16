//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2577;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2578;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta642(t12254: f64, t20293: f64, t141: f64, t12542: f64, t12543: f64, t16710: f64, t16931: f64, t17131: f64, t17140: f64, t20366: f64, t20368: f64, t20371: f64, t20373: f64, t12261: f64, t12297: f64, t16706: f64, t16876: f64, t17115: f64, t17117: f64, t20268: f64, t20274: f64, t20276: f64, t20278: f64, t20280: f64, t20322: f64, t20338: f64, t20341: f64, t20344: f64, t20347: f64, t20350: f64, t20353: f64, t20357: f64, t20359: f64, t20362: f64, t1179: f64, t1188: f64, t1196: f64, t5192: f64, t5202: f64, t5207: f64, t1189: f64, t6555: f64, t5181: f64, t5197: f64, t16988: f64, t5205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20377, t20378, t20380) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2577(t12254, t20293, t141, t12542, t12543, t16710, t16931, t17131, t17140, t20366, t20368, t20371, t20373);
        let t20382 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2578(t12261, t12297, t16706, t16876, t17115, t17117, t20268, t20274, t20276, t20278, t20280, t20322, t20338, t20341, t20344, t20347, t20350, t20353, t20357, t20359, t20362, t20380);
        let (t20384, t20386, t20388, t20390, t20391, t20393, t20394, t20396, t20397) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2579(t1179, t1188, t20382, t1196, t5192, t5202, t5207, t1189, t6555, t5181, t5197, t16988, t5205);
    (t20377, t20378, t20382, t20384, t20386, t20388, t20390, t20391, t20393, t20394, t20396, t20397)
}
