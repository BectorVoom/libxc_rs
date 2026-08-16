//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1773;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1774;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1775;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1776;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta466<F: Float>(t25266: F, t839: F, t241: F, t7036: F, t820: F, t2751: F, t159: F, t2698: F, t218: F, t816: F, t228: F, t7021: F, t802: F, t2707: F, t7025: F, t7043: F, t826: F, t2736: F, t25251: F, t25254: F, t25257: F, t25258: F, t25263: F, t25220: F, t25224: F, t25225: F, t25230: F, t25232: F, t25236: F, t25238: F, t25243: F, t25246: F, t25248: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25267, t25270) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1773::<F>(t25266, t839, t241, t7036, t820);
        let (t25271, t25273, t25276, t25277, t25278, t25279, t25280, t25282) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1774::<F>(t25270, t2751, t159, t2698, t218, t816, t228, t7021, t802, t2707, t7025, t7043, t826);
        let (t25284, t25285) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1775::<F>(t25282, t2736, t25251, t25254, t25257, t25258, t25263, t25267, t25271, t25276, t25279, t25280);
        let t25286 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1776::<F>(t25220, t25224, t25225, t25230, t25232, t25236, t25238, t25243, t25246, t25248, t25285);
    (t25267, t25270, t25273, t25276, t25277, t25278, t25282, t25284, t25286)
}
