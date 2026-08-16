//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1129/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1129(t30268: f64, t8783: f64, t31254: f64, t1479: f64, t429: f64, t1980: f64, t7476: f64, t31262: f64, t31277: f64, t31279: f64, t1089: f64, t15897: f64, t2288: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35496 = t30268 * t8783;
    let t35497 = 0.94344276868812456204e-2_f64 * t35496;
    let t35499 = 0.85748036236139473944e-3_f64 * t31254;
    let t35500 = t429 * t1479;
    let t35502 = t1980 * t7476 * t35500;
    let t35503 = 0.7145669686344956162e-3_f64 * t35502;
    let t35506 = 0.26147916666666666666e0_f64 * t31262;
    let t35507 = 0.3973125e0_f64 * t31277;
    let t35508 = 0.264875e0_f64 * t31279;
    let t35511 = t598 * t1089 * t15897 * t2288;
    (t35497, t35499, t35500, t35503, t35506, t35507, t35508, t35511)
}
