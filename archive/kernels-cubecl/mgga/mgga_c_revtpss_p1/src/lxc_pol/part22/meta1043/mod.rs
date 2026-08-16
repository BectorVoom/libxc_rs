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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3646;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3647;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3648;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3649;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3650;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3651;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3652;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1043<F: Float>(t1196: F, t20890: F, t3524: F, t16655: F, t17092: F, t16658: F, t58342: F, t16665: F, t16840: F, t16669: F, t58473: F, t16784: F, t5202: F, t12571: F, t6552: F, t43995: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F, t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t56176: F, t56183: F, t56185: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t68363: F, t68366: F, t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68454: F, t68456: F, t68459: F, t448: F, t300: F, t68946: F, t68949: F, t68951: F, t68954: F, t68956: F, t45000: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t68959, t68961, t68963, t68965, t68967, t68969) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3646::<F>(t1196, t20890, t3524, t16655, t17092, t16658, t58342, t16665, t16840, t16669, t58473, t16784, t5202);
        let (t68971, t68983) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3647::<F>(t12571, t6552, t43995, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t68997 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3648::<F>(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t69011 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3649::<F>(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t69025 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3650::<F>(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t69028, t69030, t69031) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3651::<F>(t448, t68983, t68997, t69011, t69025, t300, t68946, t68949, t68951, t68954, t68956, t68959, t68961, t68963, t68965, t68967, t68969, t68971);
        let t69044 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3652::<F>(t45000, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t69058 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3653::<F>(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
    (t68959, t68961, t68963, t68965, t68967, t68969, t68971, t69028, t69030, t69031, t69044, t69058)
}
