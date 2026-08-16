//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1253;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta335(t2966: f64, t307: f64, t302: f64, t11132: f64, t11337: f64, t944: f64, t2969: f64, t310: f64, t2979: f64, t964: f64, t3011: f64, t960: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11409, t11422, t11423, t11450, t11452, t11456, t11461) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1253(t2966, t307, t302, t11132, t11337, t944, t2969, t310, t2979, t964, t3011, t960);
    (t11409, t11422, t11423, t11450, t11452, t11456, t11461)
}
