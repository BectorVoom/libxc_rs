//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 619/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk619(t159: f64, t2522: f64, t285: f64, t545: f64, t991: f64, t281: f64, t1083: f64, t751: f64, t164: f64, t2519: f64, t547: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2932 = t2522 * t159 * t285;
    let t2936 = t991 * t545 * t285;
    let t2937 = t281 * t2936;
    let t2939 = t751 * t1083;
    let t2942 = t2519 * t164;
    let t2946 = t992 * t547;
    (t2932, t2936, t2937, t2939, t2942, t2946)
}
