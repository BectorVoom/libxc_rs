//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1044 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3654;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3655;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3656;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3657;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1044<F: Float>(t56176: F, t56183: F, t56185: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t68363: F, t68366: F, t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68454: F, t68456: F, t68459: F, t422: F, t69044: F, t69058: F, t5104: F, t3433: F, t3435: F, t1150: F, t3384: F, t16835: F, t5105: F, t16943: F, t5063: F, t43748: F, t6439: F, t12238: F, t6471: F, t20448: F, t3379: F, t1196: F, t3520: F, t3523: F, t68795: F, t12552: F, t16811: F, t6534: F, t16643: F, t5192: F, t45232: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t69072 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3654::<F>(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t69086 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3655::<F>(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t69090, t69094, t69097, t69099) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3656::<F>(t422, t69044, t69058, t69072, t69086, t5104, t3433, t3435, t1150, t3384, t16835, t5105);
        let (t69101, t69103, t69105, t69107, t69111) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3657::<F>(t16943, t5063, t43748, t6439, t12238, t6471, t20448, t3379, t1196, t3520, t3523, t68795);
        let (t69115, t69117, t69139) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3658::<F>(t1196, t12552, t16811, t6534, t16643, t5192, t45232, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
    (t69090, t69094, t69097, t69099, t69101, t69103, t69105, t69107, t69111, t69115, t69117, t69139)
}
