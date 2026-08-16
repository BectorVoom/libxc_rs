//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 937/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk937(t101: f64, t8257: f64, t159: f64, t285: f64, t7908: f64, t2522: f64, t545: f64, t281: f64, t1368: f64, t991: f64, t169: f64, t2848: f64, t301: f64, t784: f64) -> (f64, f64, f64, f64, f64) {
    let t8258 = t101 * t8257;
    let t8261 = t7908 * t159 * t285;
    let t8265 = t2522 * t545 * t285;
    let t8267 = 0.23948468020509218188e-1_f64 * t281 * t8265;
    let t8269 = t991 * t1368 * t285;
    let t8270 = t281 * t8269;
    let t8275 = 0.10809180959278284142e0_f64 * t169 * t784 * t2848 * t301;
    (t8258, t8261, t8267, t8270, t8275)
}
