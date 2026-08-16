//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 925/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk925(t1223: f64, t2488: f64, t5043: f64, t5047: f64, t5056: f64, t5071: f64, t6060: f64, t6062: f64, t6068: f64, t9623: f64, t9628: f64, t9631: f64, t9635: f64, t9742: f64, t9746: f64, t9750: f64, t9753: f64, t9756: f64) -> (f64, f64) {
    let t9777 = t2488 * t1223;
    let t9796 = t6060 - 1.5625_f64 * t5043 + t6062 + 1.5625_f64 * t5047 - 1.5625_f64 * t9623 + 3.125_f64 * t9628 - 0.5208333333333334_f64 * t9631 - 1.5625_f64 * t9635 - 1.5625_f64 * t9742 - 0.5208333333333334_f64 * t5056 - t6068 + 0.5208333333333334_f64 * t5071 + 1.5625_f64 * t9746 - 1.5625_f64 * t9750 + 0.5208333333333334_f64 * t9753 + 1.5625_f64 * t9756;
    (t9777, t9796)
}
