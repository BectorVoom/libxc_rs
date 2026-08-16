//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1165/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1165(t3308: f64, t37782: f64, t8774: f64, t11811: f64, t39378: f64, t3179: f64, t3316: f64, t10760: f64, t19877: f64, t29467: f64, t29731: f64, t6093: f64) -> (f64, f64, f64, f64, f64) {
    let t43165 = t37782 * t3308 * t8774;
    let t43167 = t39378 * t11811;
    let t43169 = t3179 * t3316;
    let t43178 = t19877 * t10760 * t29467;
    let t43181 = t6093 * t10760 * t29731;
    (t43165, t43167, t43169, t43178, t43181)
}
