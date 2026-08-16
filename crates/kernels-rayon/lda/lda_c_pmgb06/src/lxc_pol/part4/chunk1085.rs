//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1085/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1085(t1423: f64, t5233: f64, t4588: f64, t517: f64, t1925: f64, t3223: f64, t5238: f64, t1908: f64, t3220: f64, t1382: f64, t5194: f64, t1592: f64, t1962: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12603 = t1423 * t5233;
    let t12617 = t4588 * t517;
    let t12621 = t3223 * t1925;
    let t12623 = t1423 * t5238;
    let t12625 = t3220 * t1908;
    let t12631 = t5194 * t1382;
    let t12633 = t1962 * t1592;
    (t12603, t12617, t12621, t12623, t12625, t12631, t12633)
}
