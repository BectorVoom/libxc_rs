//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta105 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta105(t177: f64, t752: f64, t762: f64, t717: f64, t750: f64, t675: f64, t723: f64, t169: f64, t722: f64, t164: f64, t729: f64, t730: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2523, t2524, t2526, t2531, t2536, t2537, t2538, t2539) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk645(t177, t752, t762, t717, t750, t675, t723, t169, t722, t164, t729, t730);
    (t2523, t2524, t2526, t2531, t2536, t2537, t2538, t2539)
}
