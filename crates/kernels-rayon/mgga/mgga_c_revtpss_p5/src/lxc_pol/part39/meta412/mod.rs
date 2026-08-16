//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta412(t116926: f64, t8260: f64, t2289: f64, t655: f64, t8269: f64, t31027: f64, t31047: f64, t31032: f64, t31055: f64, t31062: f64, t101: f64, t613: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t116927, t116929, t116930, t116932, t116934, t116936, t116938) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1489(t116926, t8260, t2289, t655, t8269, t31027, t31047, t31032, t31055, t31062, t101, t613);
    (t116927, t116929, t116930, t116932, t116934, t116936, t116938)
}
