//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 366/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk366(t1152: f64, t1193: f64, t1354: f64, t117: f64, t123: f64, t191: f64, t740: f64, t315: f64, t550: f64, t109: f64, t186: f64, t55: f64) -> (f64, f64, f64, f64) {
    let t1356 = 0.0004954275694490498_f64 * t1152 * t1193 * t1354;
    let t1360 = 0.02394846802050922_f64 * t123 * t740 * t191 * t117;
    let t1363 = t123 * t315 * t550 * t117;
    let t1366 = t55 * t109 * t186;
    (t1356, t1360, t1363, t1366)
}
