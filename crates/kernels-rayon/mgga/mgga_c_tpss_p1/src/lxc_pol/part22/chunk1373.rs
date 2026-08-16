//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1373/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1373(t1791: f64, t65157: f64, t65165: f64, t19342: f64, t62348: f64, t18350: f64, t20264: f64, t62024: f64, t62259: f64, t62262: f64, t62264: f64, t62266: f64, t62270: f64, t62273: f64, t62275: f64, t62345: f64, t65182: f64) -> f64 {
    let t67349 = t1791 * t65157;
    let t67352 = t1791 * t65165;
    let t67358 = 160.0_f64 / 3.0_f64 * t62348 * t19342;
    let t67362 = 176.0_f64 / 27.0_f64 * t62259 + 176.0_f64 / 27.0_f64 * t62262 - 8.0_f64 / 9.0_f64 * t62264 - 16.0_f64 / 9.0_f64 * t62266 + 10.0_f64 / 3.0_f64 * t62024 * t20264 + 20.0_f64 / 3.0_f64 * t18350 * t67349 + 20.0_f64 / 3.0_f64 * t18350 * t67352 - 70.0_f64 * t62345 * t65182 - t67358 + 40.0_f64 / 9.0_f64 * t62270 + 16.0_f64 / 9.0_f64 * t62273 + 32.0_f64 / 9.0_f64 * t62275;
    t67362
}
