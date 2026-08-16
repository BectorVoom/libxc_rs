//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1086/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1086(t1447: f64, t4757: f64, t1989: f64, t3223: f64, t1980: f64, t883: f64, t4713: f64, t607: f64, t1710: f64, t1959: f64, t1423: f64, t4767: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12644 = t1447 * t4757;
    let t12649 = t3223 * t1989;
    let t12657 = t883 * t1980;
    let t12659 = t4713 * t607;
    let t12661 = t1959 * t1710;
    let t12677 = t1423 * t4767;
    (t12644, t12649, t12657, t12659, t12661, t12677)
}
