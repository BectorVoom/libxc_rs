//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1032/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1032(t432: f64, t5115: f64, t517: f64, t5415: f64, t161: f64, t166: f64, t529: f64, t2887: f64, t831: f64, t531: f64, t5432: f64, t1641: f64, t1848: f64) -> (f64, f64, f64, f64, f64) {
    let t12259 = t432 * t5115;
    let t12260 = 2.0_f64 / 15.0_f64 * t12259;
    let t12261 = t5415 * t517;
    let t12265 = t161 * t166 * t12261 * t529 / 10.0_f64;
    let t12267 = t831 * t2887 / 10.0_f64;
    let t12269 = t5432 * t531 / 10.0_f64;
    let t12271 = t1848 * t1641 / 5.0_f64;
    (t12260, t12265, t12267, t12269, t12271)
}
