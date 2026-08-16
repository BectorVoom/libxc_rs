//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 900/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk900(t1067: f64, t2426: f64, t143: f64, t8141: f64, t7991: f64, t4689: f64, t4692: f64, t4694: f64, t4702: f64, t4706: f64, t4708: f64, t4713: f64, t80: f64, t8973: f64, t8975: f64, t9543: f64) -> f64 {
    let t9548 = t2426 * t1067;
    let t9550 = t143 * t8141;
    let t9552 = t143 * t7991;
    let t9554 = -2.400108951976084_f64 * t8973 - t4689 + t4692 - t4694 + 14.71989892086604_f64 * t8975 - t4702 + t80 * t9543 + 18.635258017632964_f64 * t4706 + 18.635258017632964_f64 * t4708 + 0.04115066352984959_f64 * t4713 - 12.992782516386768_f64 * t9548 - 2.507382812916709_f64 * t9550 - 2.507382812916709_f64 * t9552;
    t9554
}
