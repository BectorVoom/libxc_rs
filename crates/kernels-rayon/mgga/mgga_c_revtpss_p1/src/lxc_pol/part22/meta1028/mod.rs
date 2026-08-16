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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3604;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3605;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3606;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3607;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3608;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3609;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3610;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1028(t12254: f64, t141: f64, t68265: f64, t43881: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64, t68297: f64, t68301: f64, t68305: f64, t68310: f64, t68332: f64, t68334: f64, t68336: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64, t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64, t20311: f64, t689: f64, t20307: f64, t1120: f64, t128: f64, t68317: f64, t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t1132: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t68402, t68415) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3604(t12254, t141, t68265, t43881, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t68429 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3605(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t68443 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3606(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t68454 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3607(t20311, t689);
        let t68456 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3608(t20307, t689);
        let t68459 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3609(t1120, t128, t68317);
        let t68461 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3610(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t68463, t68464, t68466) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3611(t68415, t68429, t68443, t68461, t1132, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t56230, t56236, t68389, t68393, t68397, t68399, t68402);
    (t68402, t68454, t68456, t68459, t68463, t68464, t68466)
}
