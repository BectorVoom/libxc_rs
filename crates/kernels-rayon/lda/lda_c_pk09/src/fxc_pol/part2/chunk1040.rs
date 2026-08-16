//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1040/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1040(t11277: f64, t2042: f64, t10: f64, t1729: f64, t549: f64, t11059: f64, t132: f64, t93: f64, t1672: f64, t2872: f64, t11248: f64, t1808: f64) -> (f64, f64, f64, f64, f64) {
    let t11278 = t11277 * t2042;
    let t11282 = t1729 * t10;
    let t11283 = t549 * t11282;
    let t11286 = t132 * t11059;
    let t11287 = t93 * t11286;
    let t11290 = t2872 * t1672;
    let t11292 = t1808 * t11248;
    (t11278, t11283, t11287, t11290, t11292)
}
