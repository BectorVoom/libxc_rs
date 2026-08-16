//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 857/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk857(t2727: f64, t343: f64, t8145: f64, t928: f64, t328: f64, t8148: f64, t4606: f64, t5021: f64, t8141: f64, t8143: f64, t8146: f64, t8149: f64, t8155: f64) -> (f64, f64, f64, f64) {
    let t8157 = t2727 * t343;
    let t8159 = t928 * t8145;
    let t8161 = t328 * t8148;
    let t8164 = -2.8769444444444443_f64 * t8141 + 27.618666666666666_f64 * t8143 - 10.229135802469136_f64 * t8146 + 8.950493827160495_f64 * t8149 + 3.131074074074074_f64 * t4606 + 0.0366775_f64 * t8155 - 0.58684_f64 * t8157 + 0.6520444444444444_f64 * t8159 + 0.5705388888888889_f64 * t8161 + 1.3490888888888888_f64 * t5021;
    (t8157, t8159, t8161, t8164)
}
