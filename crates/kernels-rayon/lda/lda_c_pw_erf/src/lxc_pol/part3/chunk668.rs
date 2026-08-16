//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 668/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk668(t185: f64, t4039: f64, t1401: f64, t1403: f64, t549: f64, t1466: f64, t1318: f64, t1333: f64, t212: f64) -> (f64, f64, f64, f64, f64) {
    let t4041 = 16.0_f64 / 405.0_f64 * t185 * t4039;
    let t4043 = t1401 * t549 * t1403;
    let t4044 = t1466 * t4043;
    let t4046 = 8.0_f64 / 5.0_f64 * t1318 * t4044;
    let t4048 = 1.0_f64 / t212 / t1333;
    (t4041, t4043, t4044, t4046, t4048)
}
