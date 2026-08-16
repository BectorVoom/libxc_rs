//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta574(t2434: f64, t837: f64, t25377: f64, t25431: f64, t251: f64, t25304: f64, t25374: f64, t10505: f64, t93172: f64, t2453: f64, t25398: f64, t10506: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93183, t93184, t93189, t93190, t93191, t93192, t93194, t93195) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1990(t2434, t837, t25377, t25431, t251, t25304, t25374, t10505, t93172, t2453, t25398, t10506);
    (t93183, t93184, t93189, t93190, t93191, t93192, t93194, t93195)
}
