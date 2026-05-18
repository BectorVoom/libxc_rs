//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 62/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk62<F: Float>(t128: F, t134: F, t106: F, t115: F, t123: F, t77: F, t90: F, t98: F, t110: F, t88: F, t10: F, t91: F) -> (F, F, F, F) {
    let t135 = t128 * t134;
    let t137 = -t90 * t98 / F::new(6.0) - t106 * t98 / F::new(6.0) + t115 * t98 / F::new(6.0) - F::new(0.10237773105191754) * t77 + F::new(1.0150830754383913) + F::new(0.14975624337724558) * t123 + F::new(0.006167148707670995) * t135;
    let t141 = t110 * t88;
    let t142 = t10 * t91;
    (t135, t137, t141, t142)
}
