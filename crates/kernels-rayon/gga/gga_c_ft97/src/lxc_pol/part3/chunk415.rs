//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 415/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk415(t2360: f64, t327: f64, t326: f64, t2400: f64, t1537: f64, t947: f64, t1546: f64, t89: f64, t921: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2928 = t327 * t2360;
    let t2937 = t326 * t326;
    let t2938 = 1.0_f64 / t2937;
    let t2946 = 0.19257444444444444444e0_f64 * t2400;
    let t2976 = t1537 * t947;
    let t2981 = t89 * t1546 * t921;
    (t2928, t2937, t2938, t2946, t2976, t2981)
}
