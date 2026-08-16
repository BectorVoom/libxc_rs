//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1348/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1348(t15034: f64, t859: f64, t892: f64, t1161: f64, t353: f64, t52191: f64, t53952: f64, t27729: f64, t4082: f64, t20154: f64, t3067: f64, t4207: f64, t938: f64) -> (f64, f64, f64, f64, f64) {
    let t55717 = t859 * t892 * t15034;
    let t55722 = t859 * t353 * t52191 * t1161;
    let t55726 = 7.0_f64 / 144.0_f64 * t53952;
    let t55729 = t27729 * t4082;
    let t55734 = t20154 * t3067 * t4207 * t938;
    (t55717, t55722, t55726, t55729, t55734)
}
