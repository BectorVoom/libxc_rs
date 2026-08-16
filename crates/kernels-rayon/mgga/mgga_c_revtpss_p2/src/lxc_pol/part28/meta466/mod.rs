//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1773;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1774;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1775;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1776;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta466(t25266: f64, t839: f64, t241: f64, t7036: f64, t820: f64, t2751: f64, t159: f64, t2698: f64, t218: f64, t816: f64, t228: f64, t7021: f64, t802: f64, t2707: f64, t7025: f64, t7043: f64, t826: f64, t2736: f64, t25251: f64, t25254: f64, t25257: f64, t25258: f64, t25263: f64, t25220: f64, t25224: f64, t25225: f64, t25230: f64, t25232: f64, t25236: f64, t25238: f64, t25243: f64, t25246: f64, t25248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25267, t25270) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1773(t25266, t839, t241, t7036, t820);
        let (t25271, t25273, t25276, t25277, t25278, t25279, t25280, t25282) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1774(t25270, t2751, t159, t2698, t218, t816, t228, t7021, t802, t2707, t7025, t7043, t826);
        let (t25284, t25285) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1775(t25282, t2736, t25251, t25254, t25257, t25258, t25263, t25267, t25271, t25276, t25279, t25280);
        let t25286 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1776(t25220, t25224, t25225, t25230, t25232, t25236, t25238, t25243, t25246, t25248, t25285);
    (t25267, t25270, t25273, t25276, t25277, t25278, t25282, t25284, t25286)
}
