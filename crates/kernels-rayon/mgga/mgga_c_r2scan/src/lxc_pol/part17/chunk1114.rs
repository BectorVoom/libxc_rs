//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1114/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1114(t39960: f64, t546: f64, t10729: f64, t11659: f64, t6395: f64, t10868: f64, t7614: f64, t7615: f64, t39885: f64, t8243: f64, t2605: f64, t37699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40075 = t546 * t39960;
    let t40076 = t40075 * t10729;
    let t40086 = t6395 * t11659;
    let t40090 = t7614 * t10868 * t7615;
    let t40102 = t39885 * t8243;
    let t40107 = t37699 * t2605;
    (t40075, t40076, t40086, t40090, t40102, t40107)
}
