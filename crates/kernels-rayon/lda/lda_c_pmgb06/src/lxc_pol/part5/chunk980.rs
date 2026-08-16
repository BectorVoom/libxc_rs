//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 980/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk980(t13007: f64, t6630: f64, t1636: f64, t2563: f64, t1593: f64, t2648: f64, t161: f64, t489: f64, t6460: f64, t1554: f64, t2554: f64, t517: f64, t6831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16556 = t13007 * t6630;
    let t16558 = t2563 * t1636;
    let t16563 = t1593 * t2648;
    let t16583 = t161 * t489 * t6460;
    let t16593 = t161 * t1554 * t2554;
    let t16595 = t6831 * t517;
    (t16556, t16558, t16563, t16583, t16593, t16595)
}
