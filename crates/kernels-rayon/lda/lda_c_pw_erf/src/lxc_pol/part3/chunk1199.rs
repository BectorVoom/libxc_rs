//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1199/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1199(t4199: f64, t4546: f64, t4207: f64, t4589: f64, t515: f64, t172: f64, t184: f64, t4645: f64, t496: f64, t10043: f64, t10053: f64, t10092: f64, t10098: f64, t10100: f64, t10115: f64, t13568: f64, t13571: f64, t13574: f64, t13577: f64, t13580: f64, t13585: f64, t13587: f64, t13589: f64, t13592: f64, t13595: f64, t13600: f64, t13603: f64, t13645: f64) -> (f64, f64, f64, f64, f64) {
    let t14103 = t4546 * t4199;
    let t14105 = t4546 * t4207;
    let t14107 = t4589 * t515;
    let t14108 = 8.0_f64 / 15.0_f64 * t14107;
    let t14110 = t172 * t4645 * t184;
    let t14112 = 4.0_f64 / 5.0_f64 * t14110 * t496;
    let t14131 = -t10043 + 0.04534_f64 * t13568 - 0.04534_f64 * t13571 - 0.02518888888888889_f64 * t13574 + 0.04534_f64 * t13577 - 0.06801_f64 * t13580 - 0.0012594444444444445_f64 * t13585 - 0.002099074074074074_f64 * t13587 + 0.02770777777777778_f64 * t13589 + 0.0012594444444444445_f64 * t13592 - 0.007556666666666666_f64 * t13595 + 0.005597530864197531_f64 * t13600 - 0.012594444444444445_f64 * t13603 - 0.005037777777777778_f64 * t10092 + 0.0016792592592592592_f64 * t10098 - 0.0006996913580246914_f64 * t10100 + 0.002518888888888889_f64 * t10115 + 0.034005_f64 * t13645 + 0.002518888888888889_f64 * t10053;
    (t14103, t14105, t14108, t14112, t14131)
}
