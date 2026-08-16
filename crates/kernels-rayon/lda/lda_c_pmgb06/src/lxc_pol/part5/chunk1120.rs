//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1120/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1120(t16799: f64, t13090: f64, t6705: f64, t824: f64, t6906: f64, t831: f64, t132: f64, t137: f64, t2648: f64, t4815: f64, t11877: f64, t493: f64, t6517: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20451 = t16799 / 45.0_f64;
    let t20452 = 4.0_f64 / 135.0_f64 * t13090;
    let t20454 = t6705 * t824 / 10.0_f64;
    let t20456 = t831 * t6906 / 10.0_f64;
    let t20460 = t132 * t137 * t4815 * t2648 / 10.0_f64;
    let t20463 = 2.0_f64 / 15.0_f64 * t493 * t11877 * t6517;
    (t20451, t20452, t20454, t20456, t20460, t20463)
}
