//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1043 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3646;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3647;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3648;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3649;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3650;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3651;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3652;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1043(t1196: f64, t20890: f64, t3524: f64, t16655: f64, t17092: f64, t16658: f64, t58342: f64, t16665: f64, t16840: f64, t16669: f64, t58473: f64, t16784: f64, t5202: f64, t12571: f64, t6552: f64, t43995: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64, t68297: f64, t68301: f64, t68305: f64, t68310: f64, t68332: f64, t68334: f64, t68336: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64, t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64, t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64, t448: f64, t300: f64, t68946: f64, t68949: f64, t68951: f64, t68954: f64, t68956: f64, t45000: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68959, t68961, t68963, t68965, t68967, t68969) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3646(t1196, t20890, t3524, t16655, t17092, t16658, t58342, t16665, t16840, t16669, t58473, t16784, t5202);
        let (t68971, t68983) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3647(t12571, t6552, t43995, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t68997 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3648(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t69011 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3649(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t69025 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3650(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t69028, t69030, t69031) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3651(t448, t68983, t68997, t69011, t69025, t300, t68946, t68949, t68951, t68954, t68956, t68959, t68961, t68963, t68965, t68967, t68969, t68971);
        let t69044 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3652(t45000, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t69058 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3653(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
    (t68959, t68961, t68963, t68965, t68967, t68969, t68971, t69028, t69030, t69031, t69044, t69058)
}
