//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2577;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2578;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta642<F: Float>(t12254: F, t20293: F, t141: F, t12542: F, t12543: F, t16710: F, t16931: F, t17131: F, t17140: F, t20366: F, t20368: F, t20371: F, t20373: F, t12261: F, t12297: F, t16706: F, t16876: F, t17115: F, t17117: F, t20268: F, t20274: F, t20276: F, t20278: F, t20280: F, t20322: F, t20338: F, t20341: F, t20344: F, t20347: F, t20350: F, t20353: F, t20357: F, t20359: F, t20362: F, t1179: F, t1188: F, t1196: F, t5192: F, t5202: F, t5207: F, t1189: F, t6555: F, t5181: F, t5197: F, t16988: F, t5205: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20377, t20378, t20380) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2577::<F>(t12254, t20293, t141, t12542, t12543, t16710, t16931, t17131, t17140, t20366, t20368, t20371, t20373);
        let t20382 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2578::<F>(t12261, t12297, t16706, t16876, t17115, t17117, t20268, t20274, t20276, t20278, t20280, t20322, t20338, t20341, t20344, t20347, t20350, t20353, t20357, t20359, t20362, t20380);
        let (t20384, t20386, t20388, t20390, t20391, t20393, t20394, t20396, t20397) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2579::<F>(t1179, t1188, t20382, t1196, t5192, t5202, t5207, t1189, t6555, t5181, t5197, t16988, t5205);
    (t20377, t20378, t20382, t20384, t20386, t20388, t20390, t20391, t20393, t20394, t20396, t20397)
}
