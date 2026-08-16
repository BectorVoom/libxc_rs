//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1062/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1062(t4588: f64, t517: f64, t2992: f64, t493: f64, t1925: f64, t3223: f64, t1423: f64, t5238: f64, t1908: f64, t3220: f64, t1972: f64, t2984: f64) -> (f64, f64, f64, f64, f64) {
    let t12617 = t4588 * t517;
    let t12620 = t493 * t12617 * t2992 / 9.0_f64;
    let t12621 = t3223 * t1925;
    let t12622 = 2.0_f64 / 135.0_f64 * t12621;
    let t12623 = t1423 * t5238;
    let t12624 = 4.0_f64 / 45.0_f64 * t12623;
    let t12625 = t3220 * t1908;
    let t12626 = 4.0_f64 / 45.0_f64 * t12625;
    let t12628 = t1972 * t2984 / 15.0_f64;
    (t12620, t12622, t12624, t12626, t12628)
}
