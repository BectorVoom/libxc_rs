//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 156/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk156(t132: f64, t93: f64, t481: f64, t441: f64, t453: f64, t455: f64, t463: f64, t472: f64, t478: f64, t305: f64, t467: f64) -> (f64, f64, f64) {
    let t482 = t93 * t132;
    let t483 = t481 * t482;
    let t485 = -t453 * t455 / 6.0_f64 - t463 * t455 / 6.0_f64 + t472 * t455 / 6.0_f64 - 0.10237773105191754_f64 * t441 + 1.0150830754383913_f64 + 0.14975624337724558_f64 * t478 + 0.018501446123012983_f64 * t483;
    let t489 = t467 * t305;
    (t483, t485, t489)
}
