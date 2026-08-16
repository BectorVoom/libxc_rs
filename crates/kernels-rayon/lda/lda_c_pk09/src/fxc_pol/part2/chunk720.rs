//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 720/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk720(t1782: f64, t7272: f64, t1240: f64, t2115: f64, t93: f64, t471: f64, t6700: f64, t2042: f64, t1817: f64, t2052: f64, t429: f64, t2036: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7273 = t7272 * t1782;
    let t7274 = t2115 * t1240;
    let t7275 = t93 * t7274;
    let t7276 = t7273 * t7275;
    let t7278 = t471 * t6700;
    let t7279 = t7278 * t2042;
    let t7283 = t1817 * t1817;
    let t7284 = 1.0_f64 / t7283;
    let t7286 = t2052 * t429;
    let t7292 = t2036 * t429;
    (t7273, t7275, t7276, t7279, t7284, t7286, t7292)
}
