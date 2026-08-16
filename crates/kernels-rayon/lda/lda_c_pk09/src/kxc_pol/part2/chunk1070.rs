//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1070/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1070(t1971: f64, t2855: f64, t1672: f64, t2838: f64, t11243: f64, t6991: f64, t6995: f64, t6997: f64, t7008: f64, t7015: f64, t7019: f64, t7026: f64, t7028: f64, t7032: f64) -> (f64, f64) {
    let t11600 = t2855 * t1971;
    let t11607 = t2838 * t1672;
    let t11610 = -t6991 + t11607 / 18.0_f64 - 0.04991874779241519_f64 * t11243 + t6995 + t6997 - t7008 - t7015 - t7019 + t7026 + t7028 + t7032;
    (t11600, t11610)
}
