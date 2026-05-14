//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 213/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk213<F: Float>(t666: F, t670: F, t612: F, t616: F, t626: F, t636: F, t653: F, t676: F, t681: F, t687: F, t197: F, t89: F, t148: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t777 = 18.75 * t666;
    let t778 = 12.5 * t670;
    let t782 = 1.2466946262544771 * t612;
    let t783 = 0.8311297508363181 * t616;
    let t787 = t777 + t778 + 18.75 * t676 + 18.75 * t681 - 18.75 * t687 + t782 + t783 + 1.2466946262544771 * t626 + 1.2466946262544771 * t636 - 1.2466946262544771 * t653;
    let t788 = 1.0 / t197;
    let t789 = t787 * t788;
    let t790 = t789 * t89;
    let t793 = 9.625452574583042 * t666;
    let t794 = 6.416968383055361 * t670;
    let t798 = 0.64 * t612;
    let t799 = 0.4266666666666667 * t616;
    let t803 = t793 + t794 + 9.625452574583042 * t676 + 9.625452574583042 * t681 - 9.625452574583042 * t687 + t798 + t799 + 0.64 * t626 + 0.64 * t636 - 0.64 * t653;
    let t804 = 1.0 / t148;
    let t805 = t803 * t804;
    let t806 = t805 * t89;
    (t777, t778, t782, t783, t787, t788, t789, t790, t793, t794, t798, t799, t803, t804, t805, t806)
}
