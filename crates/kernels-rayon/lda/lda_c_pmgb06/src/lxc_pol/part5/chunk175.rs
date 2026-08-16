//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 175/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk175(t451: f64, t125: f64, t147: f64, t135: f64, t146: f64, t134: f64) -> (f64, f64, f64, f64, f64) {
    let t466 = 0.035991666666666665_f64 * t451;
    let t468 = t125 * t147;
    let t471 = 0.006666666666666667_f64 * t146 * t468 * t135;
    let t472 = 1.0_f64 / t134;
    let t473 = t147 * t472;
    (t466, t468, t471, t472, t473)
}
