//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 560/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk560<F: Float>(t3223: F, t831: F, t3290: F, t3498: F, t944: F, t151: F, t3230: F, t3233: F, t49: F, t3397: F, t3409: F, t3332: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3662 = t831 * t3223;
    let t3665 = F::new(18.635258017632964) * t831 * t3290;
    let t3667 = F::new(2.507382812916709) * t944 * t3498;
    let t3668 = t151 * t3230;
    let t3670 = t151 * t3233;
    let t3676 = t49 * t49;
    let t3677 = F::new(1.0) / t3676;
    let t3692 = F::new(2.6666666666666665) * t3397;
    let t3695 = F::new(12.0) * t3409;
    let t3696 = F::new(1.0952258580751613) * t3332;
    (t3662, t3665, t3667, t3668, t3670, t3677, t3692, t3695, t3696)
}
