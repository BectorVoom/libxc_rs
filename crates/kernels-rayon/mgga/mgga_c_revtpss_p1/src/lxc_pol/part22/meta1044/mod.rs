//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1044 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3654;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3655;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3656;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3657;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1044(t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64, t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64, t422: f64, t69044: f64, t69058: f64, t5104: f64, t3433: f64, t3435: f64, t1150: f64, t3384: f64, t16835: f64, t5105: f64, t16943: f64, t5063: f64, t43748: f64, t6439: f64, t12238: f64, t6471: f64, t20448: f64, t3379: f64, t1196: f64, t3520: f64, t3523: f64, t68795: f64, t12552: f64, t16811: f64, t6534: f64, t16643: f64, t5192: f64, t45232: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t69072 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3654(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
        let t69086 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3655(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t69090, t69094, t69097, t69099) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3656(t422, t69044, t69058, t69072, t69086, t5104, t3433, t3435, t1150, t3384, t16835, t5105);
        let (t69101, t69103, t69105, t69107, t69111) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3657(t16943, t5063, t43748, t6439, t12238, t6471, t20448, t3379, t1196, t3520, t3523, t68795);
        let (t69115, t69117, t69139) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3658(t1196, t12552, t16811, t6534, t16643, t5192, t45232, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
    (t69090, t69094, t69097, t69099, t69101, t69103, t69105, t69107, t69111, t69115, t69117, t69139)
}
