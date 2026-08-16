//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2615/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2615(t11692: f64, t11697: f64, t15703: f64, t11702: f64, t5019: f64, t3516: f64, t607: f64, t1734: f64, t3493: f64, t15458: f64, t3577: f64, t15462: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53135 = t11692 * t11697 * t15703;
    let t53142 = t5019 * t11702;
    let t53144 = t3516 * t607;
    let t53149 = t1734 * t3493;
    let t53155 = t3577 * t11697 * t15458;
    let t53158 = t3577 * t11697 * t15462;
    (t53135, t53142, t53144, t53149, t53155, t53158)
}
