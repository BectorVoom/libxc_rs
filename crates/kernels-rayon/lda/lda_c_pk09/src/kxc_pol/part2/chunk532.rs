//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 532/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk532(t137: f64, t3272: f64, t1067: f64, t790: f64, t3223: f64, t957: f64, t121: f64, t3086: f64, t120: f64) -> (f64, f64, f64, f64) {
    let t3273 = t3272 * t137;
    let t3277 = t790 * t1067;
    let t3287 = t957 * t3223;
    let t3289 = t121 * t3086;
    let t3290 = t120 * t3289;
    (t3273, t3277, t3287, t3290)
}
