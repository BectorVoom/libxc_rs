//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 876/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk876(t9087: f64, t9097: f64, t9109: f64, t9124: f64, t101: f64, t89: f64, t2305: f64, t4277: f64, t1062: f64, t2336: f64, t721: f64, t1101: f64, t7991: f64) -> (f64, f64, f64, f64) {
    let t9126 = t9087 + t9097 + t9109 + t9124;
    let t9127 = t101 * t9126;
    let t9128 = t9127 * t89;
    let t9131 = t2305 * t4277;
    let t9133 = t2336 * t1062;
    let t9134 = t9133 * t721;
    let t9136 = t1101 * t7991;
    (t9128, t9131, t9134, t9136)
}
