//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 606/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk606(t1519: f64, t4979: f64, t1482: f64, t747: f64, t339: f64, t226: f64, t281: f64, t10: f64, t1240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4980 = t1519 * t4979;
    let t4981 = 22.07984838129906_f64 * t4980;
    let t4982 = t747 * t1482;
    let t4983 = t339 * t4982;
    let t4989 = t226 * t226;
    let t4990 = 1.0_f64 / t4989;
    let t4991 = t4990 * t281;
    let t4992 = t4991 * t10;
    let t4993 = t1240 * t1240;
    (t4980, t4981, t4982, t4983, t4990, t4992, t4993)
}
