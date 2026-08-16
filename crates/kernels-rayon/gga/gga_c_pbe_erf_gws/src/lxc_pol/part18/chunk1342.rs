//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1342/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1342(t14547: f64, t20842: f64, t38545: f64, t37454: f64, t6523: f64, t11461: f64, t4028: f64, t11896: f64, t4049: f64, t11475: f64, t11734: f64, t4043: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57127 = t14547 * t20842 * t38545;
    let t57130 = t14547 * t6523 * t37454;
    let t57132 = t4028 * t11461;
    let t57134 = t4049 * t11896;
    let t57138 = t4028 * t11475;
    let t57140 = t4043 * t11734;
    (t57127, t57130, t57132, t57134, t57138, t57140)
}
