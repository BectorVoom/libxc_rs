//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1138/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1138(t1489: f64, t165: f64, t1994: f64, t493: f64, t1588: f64, t1848: f64, t3447: f64, t831: f64, t10267: f64, t146: f64, t4989: f64, t9712: f64) -> (f64, f64, f64, f64, f64) {
    let t13525 = t493 * t165 * t1489 * t1994 / 5.0_f64;
    let t13527 = t1848 * t1588 / 10.0_f64;
    let t13529 = t831 * t3447 / 10.0_f64;
    let t13530 = t10267 / 45.0_f64;
    let t13532 = t146 * t9712 * t4989;
    (t13525, t13527, t13529, t13530, t13532)
}
