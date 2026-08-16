//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 792/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk792(t7848: f64, t7864: f64, t7893: f64, t7944: f64, t788: f64, t89: f64, t2152: f64, t3213: f64, t131: f64, t707: f64, t7831: f64, t2143: f64, t755: f64) -> (f64, f64, f64, f64) {
    let t7946 = t7848 + t7864 + t7893 + t7944;
    let t7947 = t7946 * t788;
    let t7948 = t7947 * t89;
    let t7951 = t3213 * t2152;
    let t7952 = t131 * t7951;
    let t7955 = t707 * t7831;
    let t7956 = t131 * t7955;
    let t7961 = t755 * t2143;
    (t7948, t7952, t7956, t7961)
}
