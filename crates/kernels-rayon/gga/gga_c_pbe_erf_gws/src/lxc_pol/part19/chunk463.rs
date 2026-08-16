//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 463/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk463(t5: f64, t814: f64, t337: f64, t2121: f64, t339: f64, t745: f64, t360: f64) -> (f64, f64, f64, f64) {
    let t2122 = t5 * t814;
    let t2123 = t337 * t2122;
    let t2124 = t2121 * t2123;
    let t2127 = t745 * t339;
    let t2132 = t339 * t360;
    (t2123, t2124, t2127, t2132)
}
