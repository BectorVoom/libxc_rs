//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1920/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1920(t28399: f64, t686: f64, t72: f64, t7058: f64, t103000: f64, t93371: f64, t25410: f64, t8011: f64, t93240: f64, t7064: f64, t28447: f64, t689: f64, t887: f64) -> (f64, f64, f64, f64, f64) {
    let t103117 = t28399 * t72 * t686;
    let t103119 = 0.14456046980341999104e-1_f64 * t7058 * t103117;
    let t103122 = t93371 * t103000;
    let t103130 = t93240 * t25410 * t8011;
    let t103136 = 0.25702851531048074406e-1_f64 * t7064 * t103117;
    let t103140 = 0.10975748638225852664e-1_f64 * t689 * t28447 * t887;
    (t103119, t103122, t103130, t103136, t103140)
}
