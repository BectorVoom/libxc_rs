//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 101/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk101(t50: f64, t73: f64, t75: f64, t80: f64, t77: f64, t8: f64, t78: f64, t76: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t294 = t73 * t50;
    let t295 = t75 * t294;
    let t296 = t295 * t80;
    let t297 = t77 * t8;
    let t298 = t78 * t297;
    let t299 = 1.0_f64 / t298;
    let t300 = t76 * t299;
    let t302 = -12.0_f64 * t296 + 12.0_f64 * t300;
    (t294, t295, t296, t297, t298, t299, t300, t302)
}
