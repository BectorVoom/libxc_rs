//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 138/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk138(t446: f64, t76: f64, t390: f64, t393: f64, t398: f64, t388: f64) -> (f64, f64, f64, f64, f64) {
    let t447 = t76 * t446;
    let t449 = 0.301925e0_f64 * t390;
    let t450 = 0.5501625e-1_f64 * t393;
    let t451 = 0.82785e-1_f64 * t398;
    let t452 = -0.86308333333333333334e0_f64 * t388 - t449 - t450 - t451;
    (t447, t449, t450, t451, t452)
}
