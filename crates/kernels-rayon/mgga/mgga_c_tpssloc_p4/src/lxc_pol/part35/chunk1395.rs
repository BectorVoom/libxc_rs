//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1395/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1395(t19451: f64, t7461: f64, t106731: f64, t1873: f64, t28002: f64, t7467: f64, t67001: f64, t28017: f64, t4028: f64, t20347: f64, t88: f64, t28007: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t106919 = 6.0_f64 * t19451 * t7461;
    let t106921 = 6.0_f64 * t106731 * t1873;
    let t106923 = 12.0_f64 * t28002 * t7467;
    let t106932 = 2.0_f64 * t67001 * t1873;
    let t106934 = 6.0_f64 * t4028 * t28017;
    let t106935 = t88 * t20347;
    let t106937 = 2.0_f64 * t106935 * t1873;
    let t106939 = 6.0_f64 * t28007 * t7467;
    (t106919, t106921, t106923, t106932, t106934, t106937, t106939)
}
