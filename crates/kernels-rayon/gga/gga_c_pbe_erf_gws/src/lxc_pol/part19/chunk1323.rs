//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1323/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1323(t11685: f64, t14015: f64, t11949: f64, t14007: f64, t11673: f64, t3065: f64, t3831: f64, t2134: f64, t14079: f64, t3827: f64, t14547: f64, t38133: f64, t6523: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57166 = t14015 * t11685;
    let t57168 = t14007 * t11949;
    let t57171 = t14007 * t11673;
    let t57173 = t3065 * t3831;
    let t57174 = t2134 * t57173;
    let t57176 = t14079 * t3827;
    let t57179 = t14547 * t6523 * t38133;
    (t57166, t57168, t57171, t57174, t57176, t57179)
}
