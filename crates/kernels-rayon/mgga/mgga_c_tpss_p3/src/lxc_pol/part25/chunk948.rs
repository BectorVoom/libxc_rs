//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 948/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk948(t11873: f64, t11875: f64, t4057: f64, t664: f64, t1023: f64, t4060: f64, t1505: f64, t2910: f64, t294: f64, t4155: f64, t11844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11940 = 4.0_f64 / 27.0_f64 * t11873;
    let t11941 = 4.0_f64 / 9.0_f64 * t11875;
    let t11942 = t664 * t4057;
    let t11943 = 2.0_f64 / 9.0_f64 * t11942;
    let t11958 = 0.19931111111111111111e0_f64 * t11942;
    let t11971 = t4060 * t1023;
    let t11976 = t1505 * t2910;
    let t11988 = 0.41203703703703703704e-2_f64 * t11873;
    let t11989 = 0.12361111111111111111e-1_f64 * t11875;
    let t11990 = 0.61805555555555555556e-2_f64 * t11942;
    let t12009 = t294 * t4155;
    let t12024 = 0.13892666666666666667e0_f64 * t11844;
    (t11940, t11941, t11942, t11943, t11958, t11971, t11976, t11988, t11989, t11990, t12009, t12024)
}
