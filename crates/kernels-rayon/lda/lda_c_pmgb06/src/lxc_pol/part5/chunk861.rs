//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 861/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk861(t109: f64, t138: f64, t3674: f64, t3676: f64, t1767: f64, t282: f64, t55: f64, t691: f64, t1062: f64, t3709: f64, t696: f64, t957: f64) -> (f64, f64, f64) {
    let t8655 = 6.87343803774119_f64 * t138 * t109 * t3674 * t3676;
    let t8659 = 0.0018989649058080863_f64 * t691 * t55 * t1767 * t282;
    let t8668 = 623.3709278045327_f64 * t696 * t3709 * t957 * t1062;
    (t8655, t8659, t8668)
}
