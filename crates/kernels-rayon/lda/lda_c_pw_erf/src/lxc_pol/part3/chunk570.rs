//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 570/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk570(t1010: f64, t155: f64, t1953: f64, t2061: f64, t2717: f64, t2720: f64, t2723: f64, t2728: f64, t2730: f64, t2732: f64, t371: f64, t363: f64, t987: f64) -> (f64, f64, f64, f64) {
    let t3046 = t155 * t1010;
    let t3058 = -4.7063_f64 * t2717 + 3.1375333333333333_f64 * t2720 - 3.6604555555555556_f64 * t2723 - 1.6068111111111112_f64 * t1953 + 0.2805166666666667_f64 * t2728 - 0.5610333333333334_f64 * t2730 - 0.6545388888888889_f64 * t2732 - 0.4630888888888889_f64 * t2061;
    let t3059 = t3058 * t371;
    let t3063 = 1.0_f64 / t987 / t363;
    (t3046, t3058, t3059, t3063)
}
