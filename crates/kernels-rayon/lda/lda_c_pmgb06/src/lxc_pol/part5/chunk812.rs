//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 812/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk812(t3172: f64, t7295: f64, t1462: f64, t493: f64, t1988: f64, t2465: f64, t1439: f64, t7284: f64, t442: f64, t439: f64, t1465: f64, t496: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7659 = t3172 * t7295;
    let t7660 = t1462 * t7659;
    let t7662 = 2.0_f64 / 9.0_f64 * t493 * t7660;
    let t7663 = t1988 * t2465;
    let t7665 = t493 * t7663 / 15.0_f64;
    let t7666 = t1439 * t7284;
    let t7667 = t442 * t7666;
    let t7669 = 2.0_f64 / 15.0_f64 * t439 * t7667;
    let t7670 = t1465 * t7295;
    let t7671 = t496 * t7670;
    (t7659, t7660, t7662, t7663, t7665, t7666, t7667, t7669, t7670, t7671)
}
