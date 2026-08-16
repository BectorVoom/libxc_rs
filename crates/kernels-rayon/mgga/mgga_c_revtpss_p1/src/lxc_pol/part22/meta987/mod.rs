//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta987 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3347;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3348;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3349;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3350;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3351;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta987(t141: f64, t2908: f64, t63306: f64, t18908: f64, t2251: f64, t930: f64, t19006: f64, t698: f64, t51957: f64, t51963: f64, t60927: f64, t51958: f64, t2857: f64, t60717: f64, t41361: f64, t41363: f64, t41610: f64, t51967: f64, t51973: f64, t51978: f64, t63299: f64, t63304: f64, t63308: f64, t128: f64, t904: f64, t18910: f64, t689: f64, t18914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63311, t63313, t63315, t63320, t63325) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3347(t141, t2908, t63306, t18908, t2251, t930, t19006, t698, t51957, t51963, t60927);
        let t63328 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3348(t51957, t51958, t60927);
        let (t63330, t63332, t63334) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3349(t2857, t60717, t141, t930, t41361, t41363, t41610, t51967, t51973, t51978, t63299, t63304, t63308, t63311, t63315, t63320, t63325, t63328);
        let t63336 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3350(t128, t63313, t904);
        let t63338 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3351(t18910, t689);
        let t63340 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3352(t18914, t689);
    (t63311, t63313, t63315, t63320, t63325, t63328, t63330, t63332, t63334, t63336, t63338, t63340)
}
