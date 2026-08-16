//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta405(t1453: f64, t8406: f64, t1843: f64, t8320: f64, t1310: f64, t31027: f64, t8395: f64, t28036: f64, t8311: f64, t1513: f64, t661: f64, t8315: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t31401, t31403, t31407, t31415, t31417, t31421) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1482(t1453, t8406, t1843, t8320, t1310, t31027, t8395, t28036, t8311, t1513, t661, t8315);
    (t31401, t31403, t31407, t31415, t31417, t31421)
}
