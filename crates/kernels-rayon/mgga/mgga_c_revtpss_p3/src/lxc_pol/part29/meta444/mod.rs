//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1661;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta444(t25266: f64, t839: f64, t241: f64, t7036: f64, t820: f64, t2751: f64, t159: f64, t2698: f64, t218: f64, t816: f64, t228: f64, t7021: f64, t802: f64, t2707: f64, t7025: f64, t7043: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25267, t25270) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1661(t25266, t839, t241, t7036, t820);
        let (t25271, t25273, t25275, t25277, t25278, t25279, t25280, t25282) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1662(t25270, t2751, t159, t2698, t218, t816, t228, t7021, t802, t2707, t7025, t7043, t826);
    (t25267, t25270, t25271, t25273, t25275, t25277, t25278, t25279, t25280, t25282)
}
