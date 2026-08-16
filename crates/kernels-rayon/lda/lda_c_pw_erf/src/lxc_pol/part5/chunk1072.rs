//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1072/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1072(t15455: f64, t11372: f64, t11374: f64, t11376: f64, t20057: f64, t20058: f64, t20059: f64, t20060: f64, t20062: f64, t20063: f64, t20067: f64, t20068: f64, t20069: f64, t8285: f64, t8290: f64, t8296: f64, t8300: f64, t8301: f64, t8356: f64) -> (f64, f64) {
    let t20070 = 60.0_f64 * t15455;
    let t20071 = -t20057 + t8285 + t20058 + t8290 + t20059 - t8296 - t20060 - t11372 - t11374 + t20062 - t8300 + t20063 + t11376 - 1.825614615114074_f64 * t8301 - t20067 + t20068 - t8356 - t20069 + t20070;
    (t20070, t20071)
}
