//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 984/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk984(t77292: f64, t3351: f64, t515: f64, t9188: f64, t9527: f64, t71207: f64, t74927: f64, t74929: f64, t74930: f64, t74932: f64, t77265: f64, t77271: f64, t77275: f64, t77279: f64, t77280: f64, t77281: f64, t77283: f64, t77286: f64, t77287: f64, t77288: f64) -> f64 {
    let t77293 = 0.12769379967989351819e-4_f64 * t77292;
    let t77296 = t3351 * t9188 * t515 * t9527;
    let t77297 = 0.25538759935978703638e-4_f64 * t77296;
    let t77298 = t77265 - t74927 + t74929 + 0.93188427318671584245e-2_f64 * t74930 - 0.15531404553111930708e-1_f64 * t74932 - t71207 - t77271 + t77275 + t77279 + t77280 - t77281 - t77283 + t77286 - t77287 + t77288 + t77293 - t77297;
    t77298
}
