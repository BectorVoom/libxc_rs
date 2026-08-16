//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta411(t1464: f64, t8283: f64, t10208: f64, t625: f64, t31036: f64, t31027: f64, t31040: f64, t31032: f64, t31059: f64, t46157: f64, t69: f64, t2289: f64, t2339: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t116899, t116912, t116913, t116915, t116917, t116919, t116926) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1488(t1464, t8283, t10208, t625, t31036, t31027, t31040, t31032, t31059, t46157, t69, t2289, t2339);
    (t116899, t116912, t116913, t116915, t116917, t116919, t116926)
}
