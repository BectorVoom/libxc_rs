//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 672/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk672(t1835: f64, t1841: f64, t1729: f64, t1837: f64, t93: f64, t1240: f64, t902: f64, t633: f64, t1836: f64, t1781: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t6272 = t1835 * t1841;
    let t6273 = t1837 * t1729;
    let t6274 = t93 * t6273;
    let t6275 = t6272 * t6274;
    let t6277 = t902 * t1240;
    let t6278 = t6277 * t633;
    let t6279 = t93 * t6278;
    let t6280 = t1836 * t6279;
    let t6282 = t1781 * t1841;
    let t6287 = t1729 * t68;
    (t6272, t6275, t6280, t6282, t6287)
}
