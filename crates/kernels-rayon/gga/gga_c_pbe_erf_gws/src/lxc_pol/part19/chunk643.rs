//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 643/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk643(t3503: f64, t590: f64, t587: f64, t2750: f64, t2754: f64, t2757: f64, t2797: f64, t2014: f64, t3481: f64, t3490: f64, t3495: f64, t3496: f64, t3498: f64, t3502: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3504 = t590 * t3503;
    let t3506 = 8.0_f64 / 45.0_f64 * t587 * t3504;
    let t3507 = 8.0_f64 / 45.0_f64 * t2750;
    let t3508 = 16.0_f64 / 45.0_f64 * t2754;
    let t3509 = 8.0_f64 / 45.0_f64 * t2757;
    let t3510 = 16.0_f64 / 45.0_f64 * t2797;
    let t3511 = t3481 + t3490 + t3495 + t3496 + t3498 - t3502 - t3506 - t3507 + t3508 - t3509 + t2014 + t3510;
    (t3504, t3506, t3507, t3508, t3509, t3510, t3511)
}
