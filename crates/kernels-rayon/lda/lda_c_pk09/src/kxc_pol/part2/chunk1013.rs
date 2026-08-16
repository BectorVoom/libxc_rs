//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1013/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1013(t1701: f64, t2703: f64, t1705: f64, t1151: f64, t2708: f64, t2707: f64, t1161: f64, t6360: f64, t6376: f64, t6381: f64, t4861: f64, t9673: f64, t9674: f64, t9675: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10983 = t2703 * t1701;
    let t10984 = t10983 * t1705;
    let t10987 = t1151 * t2708;
    let t10989 = t1701 * t2707;
    let t10990 = t10989 * t1161;
    let t10991 = t6360 * t10990;
    let t10993 = t6376 * t2707;
    let t10996 = t6381 * t2707;
    let t10997 = t10996 * t1705;
    let t11000 = t9673 + t9674 - t9675 - t4861;
    (t10984, t10987, t10991, t10993, t10997, t11000)
}
