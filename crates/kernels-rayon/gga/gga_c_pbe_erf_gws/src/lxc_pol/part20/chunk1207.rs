//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1207/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1207(t2416: f64, t3199: f64, t326: f64, t825: f64, t6148: f64, t3067: f64, t830: f64, t3916: f64, t6792: f64, t11609: f64, t2118: f64, t2494: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36129 = t3199 * t2416;
    let t36199 = t326 * t825;
    let t36200 = t36199 * t6148;
    let t36201 = t830 * t3067;
    let t36323 = t3916 * t6792;
    let t36666 = t2118 * t11609;
    let t36888 = t2494 * param_a_c;
    (t36129, t36199, t36200, t36201, t36323, t36666, t36888)
}
