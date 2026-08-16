//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 719/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk719(t2042: f64, t7255: f64, t470: f64, t902: f64, t633: f64, t93: f64, t1841: f64, t1985: f64, t1729: f64, t2115: f64, t1468: f64, t1941: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7256 = t7255 * t2042;
    let t7260 = t902 * t470;
    let t7261 = t7260 * t633;
    let t7262 = t93 * t7261;
    let t7267 = t1985 * t1841;
    let t7268 = t2115 * t1729;
    let t7269 = t93 * t7268;
    let t7272 = t1941 * t1468;
    (t7256, t7260, t7262, t7267, t7269, t7272)
}
