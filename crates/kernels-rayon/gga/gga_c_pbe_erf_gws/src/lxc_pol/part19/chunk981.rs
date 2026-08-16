//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 981/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk981(t11102: f64, t173: f64, t184: f64, t199: f64, t11039: f64, t11040: f64, t11044: f64, t11047: f64, t11050: f64, t11053: f64, t11058: f64, t11062: f64, t11063: f64, t11064: f64, t11066: f64, t11086: f64, t7915: f64, t7919: f64, t7927: f64, t7934: f64) -> (f64, f64) {
    let t11103 = t173 * t11102;
    let t11104 = t11103 * t184;
    let t11106 = 2.0_f64 / 15.0_f64 * t11104 * t199;
    let t11107 = t11039 - t7915 + t7919 + t7927 + t7934 + t11040 - t11044 - t11047 + t11050 - t11053 + t11058 + t11062 - t11063 - t11064 + t11066 + t11086 + t11106;
    (t11106, t11107)
}
