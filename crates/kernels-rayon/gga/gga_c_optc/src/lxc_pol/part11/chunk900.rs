//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 900/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk900(t123: f64, t4961: f64, t2673: f64, t3623: f64, t14360: f64, t4947: f64, t2643: f64, t4776: f64, t3634: f64, t4768: f64, t10917: f64, t1382: f64, t19: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16960 = t123 * t4961;
    let t16961 = t2673 * t16960;
    let t16962 = t3623 * t16961;
    let t16965 = t14360 * t4947;
    let t16968 = t2643 * t4776;
    let t16969 = t3634 * t16968;
    let t16975 = t2643 * t4768;
    let t16976 = t10917 * t16975;
    let t16979 = t19 * t1382;
    (t16960, t16961, t16962, t16965, t16968, t16969, t16975, t16976, t16979)
}
