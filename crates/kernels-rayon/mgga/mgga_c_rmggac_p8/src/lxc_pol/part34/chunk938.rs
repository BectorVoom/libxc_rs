//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 938/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk938(t15626: f64, t34884: f64, t3352: f64, t495: f64, t515: f64, t7230: f64, t9523: f64, t15502: f64, t3351: f64, t498: f64, t9210: f64, t321: f64, t7248: f64) -> (f64, f64, f64, f64) {
    let t76712 = t34884 * t15626;
    let t76713 = 0.12414674968878536491e-4_f64 * t76712;
    let t76717 = t7230 * t3352 * t515 * t9523 * t495;
    let t76718 = 0.15961724959986689774e-4_f64 * t76717;
    let t76722 = t3351 * t9210 * t515 * t15502 * t498;
    let t76723 = 0.85129199786595678796e-5_f64 * t76722;
    let t76727 = t3351 * t7248 * t515 * t15502 * t321;
    (t76713, t76718, t76723, t76727)
}
