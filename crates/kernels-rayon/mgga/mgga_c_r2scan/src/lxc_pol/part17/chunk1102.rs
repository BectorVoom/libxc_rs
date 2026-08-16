//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1102/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1102(t10872: f64, t11686: f64, t10891: f64, t11748: f64, t261: f64, t3304: f64, t7233: f64, t38182: f64, t927: f64, t2626: f64, t503: f64, t5119: f64) -> (f64, f64, f64, f64, f64) {
    let t39627 = t10872 * t11686;
    let t39629 = t11748 * t10891;
    let t39635 = t3304 * t261 * t7233;
    let t39637 = t38182 * t927;
    let t39640 = t503 * t5119 * t2626;
    (t39627, t39629, t39635, t39637, t39640)
}
