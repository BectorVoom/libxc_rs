//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 730/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk730(t1819: f64, t1947: f64, t2042: f64, t6319: f64, t6325: f64, t6464: f64, t1: f64, t2954: f64, t2961: f64, t2965: f64, t2250: f64, t633: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7532 = t1819 * t1947;
    let t7533 = t7532 * t2042;
    let t7537 = 1.5625_f64 * t6319;
    let t7539 = 1.0416666666666667_f64 * t6325;
    let t7545 = 0.3472222222222222_f64 * t6464;
    let t7566 = t1 * t2954;
    let t7568 = t2961 - t2965;
    let t7577 = t2250 * t633;
    (t7533, t7537, t7539, t7545, t7566, t7568, t7577)
}
