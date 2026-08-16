//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 601/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk601(t1131: f64, t540: f64, t960: f64, t1313: f64, t839: f64, t922: f64, t1137: f64, t1324: f64, t1140: f64, t1328: f64, t1322: f64, t1350: f64, t398: f64, t429: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4479 = t540 * t1131;
    let t4480 = t960 * t4479;
    let t4483 = t1313 * t839;
    let t4484 = t960 * t4483;
    let t4487 = t1313 * t922;
    let t4488 = t960 * t4487;
    let t4492 = 7.0_f64 / 72.0_f64 * t1137 * t1324;
    let t4494 = 7.0_f64 / 72.0_f64 * t1140 * t1328;
    let t4495 = t1322 * t839;
    let t4496 = t960 * t4495;
    let t4503 = t398 * t429 * t1350;
    (t4479, t4480, t4483, t4484, t4487, t4488, t4492, t4494, t4495, t4496, t4503)
}
