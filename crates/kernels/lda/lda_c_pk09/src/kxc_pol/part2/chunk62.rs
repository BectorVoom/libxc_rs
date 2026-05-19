//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 62/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk62<F: Float>(t128: F, t134: F, t106: F, t115: F, t123: F, t77: F, t90: F, t98: F, t110: F, t88: F, t10: F, t91: F) -> (F, F, F, F) {
    let t135 = t128 * t134;
    let t137 = -t90 * t98 / F::new(6.0) - t106 * t98 / F::new(6.0) + t115 * t98 / F::new(6.0) - F::cast_from(0.10237773105191754_f64) * t77 + F::cast_from(1.0150830754383913_f64) + F::cast_from(0.14975624337724558_f64) * t123 + F::cast_from(0.006167148707670995_f64) * t135;
    let t141 = t110 * t88;
    let t142 = t10 * t91;
    (t135, t137, t141, t142)
}
