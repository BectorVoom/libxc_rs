//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 784/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk784(t1296: f64, t2238: f64, t2722: f64, t2730: f64, t3632: f64, t378: f64, t5834: f64, t7043: f64, t7326: f64, t7334: f64, t7337: f64, t7351: f64, t74: f64, t787: f64) -> f64 {
    let t7353 = 6.0_f64 * t1296 * t7337 - 3.0_f64 * t2238 * t2730 + 6.0_f64 * t5834 * t2722 - 6.0_f64 * t3632 * t7334 - t378 * t7351 - 3.0_f64 * t7043 * t787 + t7326 * t74;
    t7353
}
