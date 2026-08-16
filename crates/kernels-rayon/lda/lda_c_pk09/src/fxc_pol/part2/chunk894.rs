//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 894/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk894(t1052: f64, t7991: f64, t80: f64, t8315: f64, t1094: f64, t2362: f64, t4119: f64, t86: f64, t8612: f64, t119: f64, t90: f64, t9204: f64) -> (f64, f64, f64, f64, f64) {
    let t9443 = t1052 * t7991;
    let t9445 = t8315 * t80;
    let t9446 = t9445 * t1094;
    let t9449 = t2362 * t4119;
    let t9452 = t8612 * t86;
    let t9453 = t9452 * t119;
    let t9459 = t90 * t9204;
    (t9443, t9446, t9449, t9453, t9459)
}
