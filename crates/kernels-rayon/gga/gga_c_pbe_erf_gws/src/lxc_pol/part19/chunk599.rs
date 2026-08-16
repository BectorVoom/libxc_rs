//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 599/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk599(t3131: f64, t3139: f64, t3140: f64, t3138: f64, t1136: f64, t2164: f64, t2170: f64, t2171: f64, t2168: f64, t3110: f64, t3115: f64, t3118: f64, t3122: f64, t3125: f64, t3127: f64, t3130: f64, t3136: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3142 = t3139 * t3131 * t3140;
    let t3144 = t3138 * t3142 / 48.0_f64;
    let t3145 = t2164 * t1136;
    let t3146 = 7.0_f64 / 288.0_f64 * t3145;
    let t3148 = t2170 * t3131 * t2171;
    let t3150 = t2168 * t3148 / 48.0_f64;
    let t3151 = t3110 + t3115 - t3118 + t3122 - t3125 - t3127 - t3130 - t3136 + t3144 + t3146 + t3150;
    (t3142, t3144, t3145, t3146, t3148, t3150, t3151)
}
