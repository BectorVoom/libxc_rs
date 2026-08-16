//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 602/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk602(t262: f64, t4: f64, t261: f64, t1174: f64, t1179: f64, t1178: f64, t270: f64, t4837: f64, t1151: f64, t266: f64, t1161: f64, t1197: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4840 = 1.0_f64 / t262 / t4;
    let t4842 = 0.219824473458288_f64 * t261 * t4840;
    let t4847 = t1174 * t1179;
    let t4852 = 1.0_f64 / t1178 / t270;
    let t4861 = 12.0_f64 * t4837;
    let t4875 = t1151 * t266;
    let t4878 = t1197 * t1161;
    (t4842, t4847, t4852, t4861, t4875, t4878)
}
