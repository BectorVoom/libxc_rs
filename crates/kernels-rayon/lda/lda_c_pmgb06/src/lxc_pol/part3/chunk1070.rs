//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1070/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1070(t1444: f64, t5282: f64, t10293: f64, t493: f64, t5281: f64, t12672: f64, t12676: f64, t12678: f64, t12682: f64, t12686: f64, t12690: f64, t12696: f64, t12700: f64, t12704: f64, t12708: f64) -> (f64, f64, f64) {
    let t12710 = t1444 * t5282 / 9.0_f64;
    let t12713 = t493 * t10293 * t5281 / 9.0_f64;
    let t12714 = -t12672 + t12676 - t12678 - t12682 + t12686 + t12690 - t12696 - t12700 - t12704 - t12708 - t12710 - t12713;
    (t12710, t12713, t12714)
}
