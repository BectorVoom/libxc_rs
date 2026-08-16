//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 561/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk561(t3339: f64, t3330: f64, t3444: f64, t3453: f64, t169: f64, t3086: f64, t96: f64, t839: f64, t748: f64, t846: f64, t851: f64, t902: f64, t94: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3697 = 0.18253764301252687_f64 * t3339;
    let t3706 = 0.821419393556371_f64 * t3330;
    let t3713 = 12.0_f64 * t3444;
    let t3715 = 32.0_f64 * t3453;
    let t3727 = t96 * t169 * t3086;
    let t3729 = 0.04115066352984959_f64 * t839 * t3727;
    let t3734 = t748 * t846;
    let t3736 = t748 * t851;
    let t3738 = t94 * t902;
    (t3697, t3706, t3713, t3715, t3729, t3734, t3736, t3738)
}
