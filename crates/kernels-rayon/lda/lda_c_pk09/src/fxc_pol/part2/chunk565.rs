//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 565/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk565(t3176: f64, t3767: f64, t2971: f64, t956: f64, t3194: f64, t2974: f64, t1062: f64, t789: f64, t721: f64, t3397: f64, t3409: f64, t3332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3768 = t3767 * t3176;
    let t3772 = t956 * t2971;
    let t3773 = t3772 * t3194;
    let t3775 = t3772 * t2974;
    let t3777 = t789 * t1062;
    let t3778 = t3777 * t721;
    let t3789 = 0.15124939527727072_f64 * t3397;
    let t3792 = 0.6806222787477182_f64 * t3409;
    let t3793 = 0.06033977866125206_f64 * t3332;
    (t3768, t3772, t3773, t3775, t3778, t3789, t3792, t3793)
}
