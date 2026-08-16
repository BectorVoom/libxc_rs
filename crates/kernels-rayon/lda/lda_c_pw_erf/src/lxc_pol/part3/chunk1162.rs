//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1162/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1162(t1351: f64, t588: f64, t1370: f64, t3604: f64, t3586: f64, t3589: f64, t213: f64, t11866: f64, t11909: f64, t10092: f64, t10098: f64, t10100: f64, t10115: f64, t10204: f64, t10206: f64, t10208: f64, t10210: f64, t10212: f64, t10225: f64, t10250: f64, t10252: f64, t11854: f64, t13294: f64, t13344: f64, t1371: f64, t2061: f64, t3609: f64, t3618: f64, t589: f64) -> (f64, f64) {
    let t13631 = t588 * t1351;
    let t13635 = t1370 * t3604;
    let t13639 = t3586 * t3589;
    let t13643 = t213 * t1351;
    let t13645 = t11866 * t13643 * t11909;
    let t13647 = 0.044444444444444446_f64 * t10204 - 0.022222222222222223_f64 * t10206 - 0.007407407407407408_f64 * t10208 + 0.0044444444444444444_f64 * t10210 + 0.0019753086419753087_f64 * t10212 + t10225 + 0.09597777777777777_f64 * t10092 - 0.03199259259259259_f64 * t10098 + 0.013330246913580247_f64 * t10100 - 0.047988888888888886_f64 * t10115 - 0.02666666666666667_f64 * t10250 + 0.0044444444444444444_f64 * t10252 - 0.08_f64 * t2061 * t589 * t3618 + 0.013333333333333334_f64 * t2061 * t1371 * t3609 - 0.08_f64 * t2061 * t1371 * t13344 + 0.24_f64 * t2061 * t589 * t13294 - 0.12_f64 * t11854 * t13631 * t11909 + 0.04_f64 * t11854 * t13635 * t11909 - 0.008888888888888889_f64 * t11854 * t13639 * t11909 - 0.64785_f64 * t13645;
    (t13645, t13647)
}
