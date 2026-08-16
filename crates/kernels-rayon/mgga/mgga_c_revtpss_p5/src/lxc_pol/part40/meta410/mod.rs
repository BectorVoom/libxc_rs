//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta410(t1513: f64, t2366: f64, t13514: f64, t93: f64, t10208: f64, t625: f64, t46157: f64, t69: f64, t2289: f64, t2339: f64, t655: f64, t2204: f64, t4168: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t101463, t101522, t116912, t116919, t116926, t116929, t117151) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1491(t1513, t2366, t13514, t93, t10208, t625, t46157, t69, t2289, t2339, t655, t2204, t4168);
    (t101463, t101522, t116912, t116919, t116926, t116929, t117151)
}
