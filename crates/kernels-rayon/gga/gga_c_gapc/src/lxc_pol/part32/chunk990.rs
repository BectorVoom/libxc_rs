//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 990/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk990(t11964: f64, t3137: f64, t311: f64, t9741: f64, t11417: f64, t277: f64, t128: f64, t2546: f64, t2761: f64, t1026: f64, t761: f64, t1093: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11965 = t11964 * t3137;
    let t11966 = t311 * t11965;
    let t11967 = t11966 * t9741;
    let t11969 = t277 * t11417;
    let t11970 = t2546 * t128;
    let t11971 = t2761 * t11970;
    let t11972 = t11969 * t11971;
    let t11974 = t761 * t1026;
    let t11975 = t11974 * t1093;
    (t11965, t11966, t11967, t11969, t11970, t11971, t11972, t11974, t11975)
}
