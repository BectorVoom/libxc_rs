//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 899/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk899(t10006: f64, t87: f64, t40: f64, t3360: f64, t460: f64, t4755: f64, t7997: f64, t75: f64, t472: f64, t19: f64, t3701: f64, t796: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10014 = t10006 * t87;
    let t10015 = t40 * t10014;
    let t10016 = t3360 * t460;
    let t10017 = t40 * t10016;
    let t10018 = 12.0_f64 * t4755;
    let t10019 = 2.0_f64 * t7997;
    let t10020 = t3360 * t75;
    let t10021 = t10020 * t472;
    let t10022 = 0.58482233974552040708e0_f64 * t10021;
    let t10024 = t3701 * t796 * t19;
    (t10015, t10017, t10018, t10019, t10022, t10024)
}
