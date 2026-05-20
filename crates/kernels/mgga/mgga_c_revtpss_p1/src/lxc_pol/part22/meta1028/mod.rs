//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1028 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3604;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3605;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3606;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3607;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3608;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3609;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3610;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1028<F: Float>(t12254: F, t141: F, t68265: F, t43881: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F, t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t56176: F, t56183: F, t56185: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t68363: F, t68366: F, t20311: F, t689: F, t20307: F, t1120: F, t128: F, t68317: F, t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t1132: F) -> (F, F, F, F, F, F, F) {
        let (t68402, t68415) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3604::<F>(t12254, t141, t68265, t43881, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t68429 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3605::<F>(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t68443 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3606::<F>(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t68454 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3607::<F>(t20311, t689);
        let t68456 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3608::<F>(t20307, t689);
        let t68459 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3609::<F>(t1120, t128, t68317);
        let t68461 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3610::<F>(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t68463, t68464, t68466) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3611::<F>(t68415, t68429, t68443, t68461, t1132, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t56230, t56236, t68389, t68393, t68397, t68399, t68402);
    (t68402, t68454, t68456, t68459, t68463, t68464, t68466)
}
