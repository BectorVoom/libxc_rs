//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1661;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta444<F: Float>(t25266: F, t839: F, t241: F, t7036: F, t820: F, t2751: F, t159: F, t2698: F, t218: F, t816: F, t228: F, t7021: F, t802: F, t2707: F, t7025: F, t7043: F, t826: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t25267, t25270) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1661::<F>(t25266, t839, t241, t7036, t820);
        let (t25271, t25273, t25275, t25277, t25278, t25279, t25280, t25282) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1662::<F>(t25270, t2751, t159, t2698, t218, t816, t228, t7021, t802, t2707, t7025, t7043, t826);
    (t25267, t25270, t25271, t25273, t25275, t25277, t25278, t25279, t25280, t25282)
}
