//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1022/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1022(t2101: f64, t2563: f64, t161: f64, t489: f64, t7442: f64, t132: f64, t137: f64, t2064: f64, t6734: f64, t6610: f64, t831: f64, t1450: f64, t493: f64, t7670: f64) -> (f64, f64, f64, f64, f64) {
    let t19224 = t2563 * t2101 / 10.0_f64;
    let t19226 = t161 * t489 * t7442;
    let t19227 = t19226 / 15.0_f64;
    let t19231 = t132 * t137 * t6734 * t2064 / 10.0_f64;
    let t19232 = t831 * t6610;
    let t19233 = 2.0_f64 / 15.0_f64 * t19232;
    let t19236 = 2.0_f64 / 15.0_f64 * t493 * t1450 * t7670;
    (t19224, t19227, t19231, t19233, t19236)
}
