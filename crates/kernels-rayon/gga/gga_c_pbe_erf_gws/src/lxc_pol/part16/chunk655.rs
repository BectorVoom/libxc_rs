//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 655/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk655(t3165: f64, t343: f64, t858: f64, t867: f64, t866: f64, t3131: f64, t3139: f64, t875: f64, t2168: f64, t2143: f64, t2165: f64, t2207: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3166 = t3165 * t343;
    let t3167 = t858 * t3166;
    let t3168 = t867 * t3167;
    let t3170 = t866 * t3168 / 96.0_f64;
    let t3172 = t3139 * t3131 * t875;
    let t3174 = t2168 * t3172 / 96.0_f64;
    let t3175 = 7.0_f64 / 288.0_f64 * t2143;
    let t3176 = 7.0_f64 / 288.0_f64 * t2165;
    let t3177 = 7.0_f64 / 144.0_f64 * t2207;
    (t3166, t3167, t3168, t3170, t3172, t3174, t3175, t3176, t3177)
}
