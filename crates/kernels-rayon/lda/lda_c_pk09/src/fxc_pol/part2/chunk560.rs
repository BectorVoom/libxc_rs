//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 560/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk560(t3223: f64, t831: f64, t3290: f64, t3498: f64, t944: f64, t151: f64, t3230: f64, t3233: f64, t49: f64, t3397: f64, t3409: f64, t3332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3662 = t831 * t3223;
    let t3665 = 18.635258017632964_f64 * t831 * t3290;
    let t3667 = 2.507382812916709_f64 * t944 * t3498;
    let t3668 = t151 * t3230;
    let t3670 = t151 * t3233;
    let t3676 = t49 * t49;
    let t3677 = 1.0_f64 / t3676;
    let t3692 = 2.6666666666666665_f64 * t3397;
    let t3695 = 12.0_f64 * t3409;
    let t3696 = 1.0952258580751613_f64 * t3332;
    (t3662, t3665, t3667, t3668, t3670, t3677, t3692, t3695, t3696)
}
