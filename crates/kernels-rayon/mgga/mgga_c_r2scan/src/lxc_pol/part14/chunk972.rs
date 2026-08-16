//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 972/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk972(t10956: f64, t10964: f64, t10969: f64, t10952: f64, t10960: f64, t11335: f64, t11340: f64, t11344: f64, t11347: f64, t11350: f64, t11352: f64, t10973: f64) -> (f64, f64, f64, f64, f64) {
    let t11372 = 0.162600798888400151e-2_f64 * t10956;
    let t11374 = 0.30487649791575028312e-3_f64 * t10964;
    let t11375 = 0.68400385060046895e-6_f64 * t10969;
    let t11376 = 0.86737941314158990616e-4_f64 * t10952 + t11372 - 0.60975299583150056624e-3_f64 * t10960 - t11374 - t11335 + t11340 - t11344 - t11347 - t11350 - t11352 + t11375;
    let t11377 = 0.60975299583150056624e-3_f64 * t10973;
    (t11372, t11374, t11375, t11376, t11377)
}
