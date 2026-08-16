//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 590/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk590(t1067: f64, t806: f64, t3223: f64, t980: f64, t3290: f64, t984: f64, t204: f64, t4091: f64, t737: f64, t143: f64, t3230: f64, t3233: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4531 = t806 * t1067;
    let t4563 = t980 * t3223;
    let t4566 = 4.937333717448355_f64 * t980 * t3290;
    let t4567 = t984 * t3223;
    let t4570 = 3.7610742193750633_f64 * t984 * t3290;
    let t4571 = t4091 * t204;
    let t4572 = t4571 * t737;
    let t4574 = t143 * t3230;
    let t4576 = t143 * t3233;
    (t4531, t4563, t4566, t4567, t4570, t4572, t4574, t4576)
}
