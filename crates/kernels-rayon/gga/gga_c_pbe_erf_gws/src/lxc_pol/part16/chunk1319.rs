//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1319/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1319(t353: f64, t55151: f64, t859: f64, t938: f64, t53424: f64, t14188: f64, t14881: f64, t14888: f64, t14922: f64, t19895: f64, t22379: f64, t2408: f64, t2409: f64, t4385: f64, t52263: f64, t52266: f64, t52270: f64, t53378: f64, t53386: f64, t53395: f64, t55137: f64, t55142: f64, t55145: f64, t6781: f64, t6793: f64, t827: f64, t9218: f64, t9283: f64) -> f64 {
    let t55154 = t859 * t353 * t55151 * t938;
    let t55161 = 35.0_f64 / 288.0_f64 * t53424;
    let t55162 = t53378 / 384.0_f64 + 7.0_f64 / 48.0_f64 * t52263 + t2408 * t2409 * t6781 * t14922 / 24.0_f64 + t53386 / 12.0_f64 + 7.0_f64 / 144.0_f64 * t52266 + 7.0_f64 / 288.0_f64 * t52270 - t53395 / 384.0_f64 - t4385 * t55137 / 48.0_f64 - t827 * t55142 / 48.0_f64 - 35.0_f64 / 432.0_f64 * t55145 + t2408 * t9283 * t14881 * t9218 / 8.0_f64 + t6793 * t55154 / 24.0_f64 + t19895 * t14888 / 48.0_f64 + t22379 * t14188 / 24.0_f64 - t55161;
    t55162
}
