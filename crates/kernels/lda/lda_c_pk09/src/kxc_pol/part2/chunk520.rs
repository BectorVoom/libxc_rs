//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 520/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk520<F: Float>(t161: F, t3233: F, t3397: F, t3409: F, t3332: F, t3339: F, t3330: F, t3444: F, t3453: F, t3223: F, t831: F, t3290: F, t3498: F, t944: F, t151: F, t3230: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3616 = t161 * t3233;
    let t3629 = 2.1389894610184537 * t3397;
    let t3632 = 9.625452574583042 * t3409;
    let t3633 = 0.8533333333333334 * t3332;
    let t3634 = 0.14222222222222222 * t3339;
    let t3643 = 0.64 * t3330;
    let t3650 = 9.625452574583042 * t3444;
    let t3652 = 25.667873532221446 * t3453;
    let t3662 = t831 * t3223;
    let t3665 = 18.635258017632964 * t831 * t3290;
    let t3667 = 2.507382812916709 * t944 * t3498;
    let t3668 = t151 * t3230;
    (t3616, t3629, t3632, t3633, t3634, t3643, t3650, t3652, t3662, t3665, t3667, t3668)
}
