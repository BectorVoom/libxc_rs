//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 210/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk210(t1010: f64, t1011: f64, t361: f64, t363: f64, t336: f64, t371: f64, t368: f64, t376: f64, t61: f64, t122: f64, t374: f64, t370: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1012 = t1010 * t1011;
    let t1013 = t361 * t361;
    let t1014 = 1.0_f64 / t1013;
    let t1015 = t1014 * t363;
    let t1016 = t371 * t336;
    let t1017 = 1.0_f64 / t1016;
    let t1018 = t368 * t1017;
    let t1019 = t1015 * t1018;
    let t1020 = t1012 * t1019;
    let t1021 = t61 * t376;
    let t1036 = t374 * t122 * t376;
    let t1038 = t370 * t1036 / 4608.0_f64;
    (t1013, t1014, t1015, t1017, t1019, t1020, t1021, t1036, t1038)
}
