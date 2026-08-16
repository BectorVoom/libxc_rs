//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1254;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1255;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta280(t30: f64, t525: f64, t2: f64, t22: f64, t33: f64, t527: f64, t2490: f64, t737: f64, t2492: f64, t744: f64, t185: f64, t2494: f64, t1340: f64, t2516: f64, t4038: f64, t9283: f64, t9286: f64, t9289: f64, t9292: f64, t9296: f64, t9298: f64, t9300: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9335, t9342, t9350, t9367, t9368) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1254(t30, t525, t2, t22, t33, t527, t2490, t737, t2492, t744);
        let (t9371, t9372, t9374, t9375, t9385) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1255(t185, t2494, t9367, t9368, t1340, t2516, t4038, t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303);
    (t9335, t9342, t9350, t9367, t9368, t9371, t9372, t9374, t9375, t9385)
}
