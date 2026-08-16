//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1108/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1108(t40061: f64, t565: f64, t10728: f64, t7258: f64, t39960: f64, t546: f64, t10729: f64, t11659: f64, t6395: f64, t10868: f64, t7614: f64, t7615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40066 = t565 * t40061;
    let t40070 = t10728 * t7258;
    let t40075 = t546 * t39960;
    let t40076 = t40075 * t10729;
    let t40077 = 0.47609969197673950972e-2_f64 * t40076;
    let t40086 = t6395 * t11659;
    let t40087 = 0.46574606203128791246e-1_f64 * t40086;
    let t40090 = t7614 * t10868 * t7615;
    (t40066, t40070, t40075, t40077, t40087, t40090)
}
