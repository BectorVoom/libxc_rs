//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1030/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1030(t40561: f64, t42429: f64, t42432: f64, t42438: f64, t42442: f64, t42444: f64, t42455: f64, t42456: f64, t42457: f64, t42459: f64, t42460: f64, t42461: f64, t48205: f64, t48208: f64, t48211: f64, t48217: f64, t48221: f64, t48225: f64, t48231: f64, t48233: f64) -> f64 {
    let t50925 = t42429 - t42432 - 0.18404604457881959845e2_f64 * t48205 - 0.29792074959875355558e-1_f64 * t48208 + 0.13803453343411469884e2_f64 * t48211 + t42438 + t42442 - t42444 - 0.12269736305254639897e2_f64 * t48217 - 0.92023022289409799224e1_f64 * t48221 - 0.92023022289409799224e1_f64 * t48225 - t48231 + t48233 + t42455 - t42456 + t42457 - 0.59584149919750711115e-1_f64 * t40561 + t42459 - t42460 + t42461;
    t50925
}
