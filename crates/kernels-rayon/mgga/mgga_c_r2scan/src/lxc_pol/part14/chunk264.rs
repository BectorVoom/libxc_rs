//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 264/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk264(t308: f64, t810: f64, t513: f64, t295: f64, t299: f64, t305: f64, t803: f64, t807: f64, t320: f64) -> (f64, f64, f64, f64, f64) {
    let t811 = t308 * t810;
    let t814 = t513 / 3.0_f64;
    let t815 = -5.0_f64 / 3.0_f64 * t803 * t299 + 5.0_f64 / 3.0_f64 * t295 * t807 + 5.0_f64 / 3.0_f64 * t305 * t811 + t814;
    let t817 = t320 * t320;
    let t818 = 1.0_f64 / t817;
    (t811, t814, t815, t817, t818)
}
