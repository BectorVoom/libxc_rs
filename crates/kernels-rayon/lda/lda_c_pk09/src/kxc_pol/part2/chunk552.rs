//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 552/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk552(t3444: f64, t62: f64, t3422: f64, t660: f64, t665: f64) -> (f64, f64, f64) {
    let t3445 = 18.75_f64 * t3444;
    let t3446 = t62 * t62;
    let t3447 = 1.0_f64 / t3446;
    let t3452 = t660 * t3422;
    let t3453 = t3452 * t665;
    (t3445, t3447, t3453)
}
