//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 995/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk995(t3131: f64, t3139: f64, t6360: f64, t3138: f64, t3123: f64, t6411: f64, t3184: f64, t6484: f64, t1114: f64, t6701: f64, t2124: f64, t3128: f64, t6563: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8921 = t3139 * t3131 * t6360;
    let t8923 = t3138 * t8921 / 48.0_f64;
    let t8925 = t3123 * t6411 / 96.0_f64;
    let t8927 = 7.0_f64 / 72.0_f64 * t6484 * t3184;
    let t8928 = t1114 * t6701;
    let t8930 = t8928 * t2124 / 48.0_f64;
    let t8932 = t3128 * t6563 / 16.0_f64;
    (t8921, t8923, t8925, t8927, t8930, t8932)
}
