//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 995/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk995(t161: f64, t489: f64, t6730: f64, t132: f64, t435: f64, t6226: f64, t1600: f64, t6904: f64, t2485: f64, t3220: f64, t1423: f64, t6250: f64) -> (f64, f64, f64, f64, f64) {
    let t17938 = t161 * t489 * t6730;
    let t17960 = t132 * t435 * t6226;
    let t17964 = t1600 * t6904;
    let t17982 = t3220 * t2485;
    let t17984 = t1423 * t6250;
    (t17938, t17960, t17964, t17982, t17984)
}
