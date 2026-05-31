//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 561/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk561<F: Float>(t3339: F, t3330: F, t3444: F, t3453: F, t169: F, t3086: F, t96: F, t839: F, t748: F, t846: F, t851: F, t902: F, t94: F) -> (F, F, F, F, F, F, F, F) {
    let t3697 = F::cast_from(0.18253764301252687_f64) * t3339;
    let t3706 = F::cast_from(0.821419393556371_f64) * t3330;
    let t3713 = F::cast_from(12.0_f64) * t3444;
    let t3715 = F::cast_from(32.0_f64) * t3453;
    let t3727 = t96 * t169 * t3086;
    let t3729 = F::cast_from(0.04115066352984959_f64) * t839 * t3727;
    let t3734 = t748 * t846;
    let t3736 = t748 * t851;
    let t3738 = t94 * t902;
    (t3697, t3706, t3713, t3715, t3729, t3734, t3736, t3738)
}
