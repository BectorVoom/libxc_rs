//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 713/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk713(t309: f64, t454: f64, t4977: f64, t2040: f64, t6791: f64, t6803: f64, t2037: f64, t6253: f64, t2056: f64, t7017: f64, t633: f64, t6611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7030 = t309 * t454 * t4977;
    let t7032 = t2040 * t7030 / 6.0_f64;
    let t7041 = 0.037002892246025966_f64 * t6791;
    let t7045 = 0.14975624337724558_f64 * t6803;
    let t7049 = t2037 * t6253;
    let t7053 = t2056 * t7030 / 6.0_f64;
    let t7064 = t2040 * t7017 / 9.0_f64;
    let t7066 = t309 * t6611 * t633;
    (t7030, t7032, t7041, t7045, t7049, t7053, t7064, t7066)
}
