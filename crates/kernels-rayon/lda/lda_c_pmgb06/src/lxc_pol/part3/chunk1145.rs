//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1145/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1145(t12535: f64, t495: f64, t5065: f64, t12539: f64, t5069: f64, t10269: f64, t10273: f64, t10286: f64, t161: f64, t1639: f64, t166: f64, t4935: f64) -> (f64, f64, f64, f64, f64) {
    let t13672 = t5065 * t12535 * t495;
    let t13675 = 8.0_f64 / 15.0_f64 * t13672 * t5069 * t12539;
    let t13676 = 4.0_f64 / 45.0_f64 * t10269;
    let t13677 = 4.0_f64 / 45.0_f64 * t10273;
    let t13678 = 4.0_f64 / 45.0_f64 * t10286;
    let t13682 = t161 * t166 * t1639 * t4935 / 10.0_f64;
    (t13675, t13676, t13677, t13678, t13682)
}
