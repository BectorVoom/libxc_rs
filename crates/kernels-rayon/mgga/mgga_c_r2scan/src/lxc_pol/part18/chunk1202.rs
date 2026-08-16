//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1202/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1202(t10760: f64, t22868: f64, t29471: f64, t2196: f64, t29779: f64, t3308: f64, t10710: f64, t30119: f64, t37586: f64, t3602: f64, t37755: f64, t7605: f64) -> (f64, f64, f64, f64) {
    let t43348 = t22868 * t10760 * t29471;
    let t43351 = t2196 * t3308 * t29779;
    let t43356 = t37586 * t10710 * t30119;
    let t43359 = t37755 * t3602 * t7605;
    (t43348, t43351, t43356, t43359)
}
