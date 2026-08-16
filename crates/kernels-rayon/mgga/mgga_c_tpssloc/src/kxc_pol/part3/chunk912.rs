//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 912/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk912(t1032: f64, t3082: f64, t2393: f64, t374: f64, t376: f64, t370: f64, t3158: f64, t964: f64, t10335: f64, t221: f64, t339: f64, t2955: f64, t995: f64) -> (f64, f64, f64, f64, f64) {
    let t10372 = t1032 * t3082;
    let t10375 = t374 * t2393 * t376;
    let t10377 = t370 * t10375 / 10368.0_f64;
    let t10381 = t964 * t3158;
    let t10383 = t221 * t10335;
    let t10385 = 5.0_f64 / 1296.0_f64 * t339 * t10383;
    let t10388 = t2955 * t995;
    (t10372, t10377, t10381, t10385, t10388)
}
