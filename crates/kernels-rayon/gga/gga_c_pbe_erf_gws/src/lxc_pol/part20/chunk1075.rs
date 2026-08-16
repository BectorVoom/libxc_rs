//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1075/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1075(t12074: f64, t12076: f64, t11478: f64, t3139: f64, t3140: f64, t3138: f64, t875: f64, t2168: f64, t11994: f64, t2255: f64, t2279: f64, t3820: f64, t6484: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12078 = t12074 * t12076 / 96.0_f64;
    let t12080 = t3139 * t11478 * t3140;
    let t12082 = t3138 * t12080 / 16.0_f64;
    let t12084 = t3139 * t11478 * t875;
    let t12086 = t2168 * t12084 / 96.0_f64;
    let t12088 = t2255 * t11994 * t2279;
    let t12092 = t6484 * t3820;
    (t12078, t12080, t12082, t12084, t12086, t12088, t12092)
}
