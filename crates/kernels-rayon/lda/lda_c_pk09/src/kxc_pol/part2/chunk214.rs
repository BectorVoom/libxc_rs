//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 214/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk214(t666: f64, t670: f64, t612: f64, t616: f64, t626: f64, t636: f64, t653: f64, t676: f64, t681: f64, t687: f64, t197: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t777 = 18.75_f64 * t666;
    let t778 = 12.5_f64 * t670;
    let t782 = 1.2466946262544771_f64 * t612;
    let t783 = 0.8311297508363181_f64 * t616;
    let t787 = t777 + t778 + 18.75_f64 * t676 + 18.75_f64 * t681 - 18.75_f64 * t687 + t782 + t783 + 1.2466946262544771_f64 * t626 + 1.2466946262544771_f64 * t636 - 1.2466946262544771_f64 * t653;
    let t788 = 1.0_f64 / t197;
    let t789 = t787 * t788;
    let t790 = t789 * t89;
    (t777, t778, t782, t783, t787, t788, t789, t790)
}
