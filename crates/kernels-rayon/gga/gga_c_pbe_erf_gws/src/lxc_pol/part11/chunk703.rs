//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 703/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk703(t3342: f64, t4757: f64, t3351: f64, t4767: f64, t3360: f64, t460: f64, t40: f64, t75: f64, t472: f64, t19: f64, t3701: f64, t796: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9981 = t4757 * t3342;
    let t9993 = t4767 * t3351;
    let t10016 = t3360 * t460;
    let t10017 = t40 * t10016;
    let t10020 = t3360 * t75;
    let t10021 = t10020 * t472;
    let t10024 = t3701 * t796 * t19;
    (t9981, t9993, t10016, t10017, t10020, t10021, t10024)
}
