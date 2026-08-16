//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1038/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1038(t3115: f64, t3433: f64, t3100: f64, t113: f64, t29222: f64, t3090: f64, t481: f64, t9235: f64, t2526: f64, t2841: f64, t3216: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29451 = t3433 * t3115;
    let t29454 = t3433 * t3100;
    let t29467 = t29222 * t113;
    let t29471 = t3090 * t481;
    let t29496 = t9235 * t481;
    let t29500 = t2841 * t2526;
    let t29699 = t3216 * t494;
    (t29451, t29454, t29467, t29471, t29496, t29500, t29699)
}
