//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 156/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk156<F: Float>(t132: F, t93: F, t481: F, t441: F, t453: F, t455: F, t463: F, t472: F, t478: F, t305: F, t467: F) -> (F, F, F) {
    let t482 = t93 * t132;
    let t483 = t481 * t482;
    let t485 = -t453 * t455 / F::cast_from(6.0_f64) - t463 * t455 / F::cast_from(6.0_f64) + t472 * t455 / F::cast_from(6.0_f64) - F::cast_from(0.10237773105191754_f64) * t441 + F::cast_from(1.0150830754383913_f64) + F::cast_from(0.14975624337724558_f64) * t478 + F::cast_from(0.018501446123012983_f64) * t483;
    let t489 = t467 * t305;
    (t483, t485, t489)
}
