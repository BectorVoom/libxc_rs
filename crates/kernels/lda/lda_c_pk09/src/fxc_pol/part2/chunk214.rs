//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 214/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk214<F: Float>(t666: F, t670: F, t612: F, t616: F, t626: F, t636: F, t653: F, t676: F, t681: F, t687: F, t197: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t777 = F::new(18.75) * t666;
    let t778 = F::new(12.5) * t670;
    let t782 = F::new(1.2466946262544771) * t612;
    let t783 = F::new(0.8311297508363181) * t616;
    let t787 = t777 + t778 + F::new(18.75) * t676 + F::new(18.75) * t681 - F::new(18.75) * t687 + t782 + t783 + F::new(1.2466946262544771) * t626 + F::new(1.2466946262544771) * t636 - F::new(1.2466946262544771) * t653;
    let t788 = F::new(1.0) / t197;
    let t789 = t787 * t788;
    let t790 = t789 * t89;
    (t777, t778, t782, t783, t787, t788, t789, t790)
}
