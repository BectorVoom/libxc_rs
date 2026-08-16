//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1175/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1175(t3594: f64, t39745: f64, t10760: f64, t2147: f64, t28005: f64, t11727: f64, t11748: f64, t22790: f64, t31064: f64, t22868: f64, t29471: f64, t2196: f64, t29779: f64, t3308: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43332 = t39745 * t3594;
    let t43335 = t2147 * t10760 * t28005;
    let t43337 = t11748 * t11727;
    let t43340 = t22790 * t10760 * t31064;
    let t43348 = t22868 * t10760 * t29471;
    let t43351 = t2196 * t3308 * t29779;
    (t43332, t43335, t43337, t43340, t43348, t43351)
}
