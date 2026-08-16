//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2670/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2670(t53796: f64, t53798: f64, t39305: f64, t1799: f64, t3914: f64, t12477: f64, t20067: f64, t3734: f64, t3918: f64, t39261: f64, t39266: f64, t39304: f64, t39309: f64, t39312: f64, t39316: f64, t5126: f64, t5161: f64, t6330: f64) -> (f64, f64, f64, f64) {
    let t56114 = 0.46785788981077169656e1_f64 * t53796;
    let t56115 = 0.70178683471615754484e1_f64 * t53798;
    let t56119 = 0.20779030926817756511e3_f64 * t39305;
    let t56120 = t1799 * t3914;
    let t56124 = -6.0_f64 * t12477 * t5126 * t6330 + 6.0_f64 * t20067 * t3734 * t5126 - 6.0_f64 * t3918 * t5161 * t56120 - t39261 - t39266 - t39304 - t39309 + t39312 + t39316 + t56114 - t56115 + t56119;
    (t56114, t56115, t56119, t56124)
}
