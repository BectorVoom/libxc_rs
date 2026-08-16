//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 697/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk697(t498: f64, t6601: f64, t1672: f64, t1979: f64, t1975: f64, t129: f64, t132: f64, t1906: f64, t1904: f64, t1671: f64, t1920: f64, t1949: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6603 = 0.8357942709722364_f64 * t498 * t6601;
    let t6604 = t1979 * t1672;
    let t6606 = t1975 * t1672;
    let t6611 = t129 * t132;
    let t6612 = t6611 * t1906;
    let t6613 = t1904 * t6612;
    let t6615 = t1671 * t1920;
    let t6616 = t1904 * t6615;
    let t6620 = t1671 * t1949;
    (t6603, t6604, t6606, t6611, t6613, t6616, t6620)
}
