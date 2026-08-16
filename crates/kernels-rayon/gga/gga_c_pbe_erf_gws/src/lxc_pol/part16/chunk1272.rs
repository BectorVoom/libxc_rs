//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1272/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1272(t14011: f64, t9393: f64, t14498: f64, t9401: f64, t3179: f64, t51291: f64, t854: f64, t14015: f64, t9651: f64, t9517: f64, t9397: f64, t3228: f64, t51465: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54096 = t14011 * t9393;
    let t54098 = t14498 * t9401;
    let t54101 = t51291 * t3179;
    let t54102 = t854 * t54101;
    let t54107 = t14015 * t9651;
    let t54109 = t14015 * t9517;
    let t54111 = t14011 * t9397;
    let t54113 = t51465 * t3228;
    (t54096, t54098, t54102, t54107, t54109, t54111, t54113)
}
