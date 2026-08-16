//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 898/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk898(t120: f64, t133: f64, t2869: f64, t1870: f64, t3357: f64, t5639: f64, t8939: f64, t8942: f64, t8945: f64, t9024: f64, t9021: f64, t2775: f64, t452: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9083 = 2.9801938271604937_f64 * t133 * t2869 * t120;
    let t9094 = t1870 * t5639 * t3357;
    let t9096 = t133 * t8939;
    let t9098 = t133 * t8942;
    let t9100 = t133 * t8945;
    let t9104 = t133 * t9024;
    let t9110 = t133 * t9021;
    let t9118 = t452 * t2775;
    (t9083, t9094, t9096, t9098, t9100, t9104, t9110, t9118)
}
