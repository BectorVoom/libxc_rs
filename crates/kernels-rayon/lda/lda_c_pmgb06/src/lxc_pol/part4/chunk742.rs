//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 742/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk742(t1872: f64, t464: f64, t477: f64, t137: f64, t132: f64, t2108: f64, t432: f64, t1848: f64, t531: f64, t1397: f64, t802: f64, t1887: f64, t479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4815 = t1872 * t464;
    let t4816 = t4815 * t477;
    let t4817 = t137 * t4816;
    let t4819 = t132 * t4817 / 15.0_f64;
    let t4821 = t432 * t2108 / 15.0_f64;
    let t4823 = t1848 * t531 / 15.0_f64;
    let t4825 = t802 * t1397 / 15.0_f64;
    let t4827 = t1887 * t479 / 15.0_f64;
    (t4815, t4816, t4817, t4819, t4821, t4823, t4825, t4827)
}
