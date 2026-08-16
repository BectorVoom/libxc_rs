//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 917/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk917(t1244: f64, t2061: f64, t539: f64, t331: f64, t3478: f64, t1250: f64, t1275: f64, t933: f64, t1269: f64, t3524: f64, t3520: f64, t325: f64, t3504: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9762 = t1244 * t1244;
    let t9763 = 1.0_f64 / t9762;
    let t9772 = t2061 * t539;
    let t9774 = t331 * t3478;
    let t9777 = 1.0_f64 / t1244 / t1250;
    let t9782 = t933 * t1275;
    let t9784 = t933 * t1269;
    let t9786 = t331 * t3524;
    let t9788 = t331 * t3520;
    let t9806 = t325 * t3504;
    (t9763, t9772, t9774, t9777, t9782, t9784, t9786, t9788, t9806)
}
