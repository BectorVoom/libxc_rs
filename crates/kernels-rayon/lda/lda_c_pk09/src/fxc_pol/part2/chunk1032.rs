//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1032/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1032(t1240: f64, t2743: f64, t93: f64, t6272: f64, t2888: f64, t902: f64, t633: f64, t2739: f64, t6977: f64, t2738: f64, t6258: f64, t429: f64, t6262: f64) -> (f64, f64, f64, f64, f64) {
    let t11163 = t2743 * t1240;
    let t11164 = t93 * t11163;
    let t11165 = t6272 * t11164;
    let t11167 = t902 * t2888;
    let t11168 = t11167 * t633;
    let t11172 = t2739 * t6977;
    let t11175 = t6258 * t2738;
    let t11176 = t6262 * t429;
    (t11165, t11168, t11172, t11175, t11176)
}
