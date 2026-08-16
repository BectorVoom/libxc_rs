//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 371/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk371(t1696: f64, t83: f64, t208: f64, t213: f64, t580: f64, t97: f64, t588: f64, t604: f64, t607: f64, t109: f64, t131: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1697 = t83 * t1696;
    let t1698 = t1697 * t208;
    let t1700 = t1698 * t213 / 3.0_f64;
    let t1701 = t580 * t97;
    let t1703 = 0.12155555555555556_f64 * t1701 * t588;
    let t1708 = t604 * t607;
    let t1710 = t131 * t109;
    (t1697, t1698, t1700, t1701, t1703, t1708, t1710)
}
