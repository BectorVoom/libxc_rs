//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 603/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk603(t1077: f64, t513: f64, t1083: f64, t398: f64, t879: f64, t1095: f64, t384: f64, t1131: f64, t506: f64, t1441: f64, t997: f64, t839: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4516 = t513 * t1077;
    let t4518 = t398 * t1083 * t4516;
    let t4521 = t513 * t879;
    let t4523 = t398 * t1095 * t4521;
    let t4524 = t384 * t4523;
    let t4526 = t506 * t1131;
    let t4528 = t398 * t1083 * t4526;
    let t4532 = 0.16006300097412701803e-1_f64 * t997 * t1441;
    let t4533 = t513 * t839;
    (t4516, t4518, t4521, t4523, t4524, t4526, t4528, t4532, t4533)
}
