//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 896/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk896(t3319: f64, t3333: f64, t3325: f64, t184: f64, t186: f64, t247: f64, t187: f64, t3024: f64, t3389: f64, t534: f64, t540: f64, t3027: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10681 = t3319 * t3333;
    let t10684 = 0.04472697096444135_f64 * t3325 * t3333;
    let t10687 = 0.004413481481481482_f64 * t184 * t247 * t186;
    let t10690 = 16.0_f64 / 3.0_f64 * t3024 * t187;
    let t10693 = t534 * t3389;
    let t10696 = 0.004413481481481482_f64 * t540 * t3389;
    let t10699 = t3027 * t187;
    (t10681, t10684, t10687, t10690, t10693, t10696, t10699)
}
