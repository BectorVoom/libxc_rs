//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta593(t1455: f64, t7700: f64, t1464: f64, t7690: f64, t2167: f64, t4168: f64, t27089: f64, t575: f64, t116: f64, t26799: f64, t10368: f64, t55: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t96684, t96690, t96692, t96694, t96706, t96733) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2054(t1455, t7700, t1464, t7690, t2167, t4168, t27089, t575, t116, t26799, t10368, t55);
    (t96684, t96690, t96692, t96694, t96706, t96733)
}
