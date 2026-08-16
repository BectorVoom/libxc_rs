//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 862/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk862(t3947: f64, t654: f64, t1090: f64, t1101: f64, t1023: f64, t1035: f64, t350: f64, t634: f64, t1040: f64, t1043: f64, t632: f64, t138: f64, t3875: f64, t3885: f64) -> (f64, f64, f64, f64, f64) {
    let t8614 = t3947 * t654;
    let t8616 = t1101 * t1090;
    let t8621 = 0.4274_f64 * t350 * t1023 * t1035 * t634;
    let t8626 = 3.436719018870595_f64 * t350 * t1040 * t1035 * t1043 * t632;
    let t8629 = 0.4274_f64 * t138 * t3885 * t3875;
    (t8614, t8616, t8621, t8626, t8629)
}
