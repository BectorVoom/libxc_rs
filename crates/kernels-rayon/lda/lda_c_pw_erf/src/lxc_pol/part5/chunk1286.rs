//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1286/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1286(t173: f64, t184: f64, t199: f64, t23004: f64, t23025: f64, t18681: f64, t826: f64, t15144: f64, t15146: f64, t15147: f64, t19318: f64, t19320: f64, t22967: f64, t22971: f64, t22975: f64, t22978: f64, t22981: f64, t22983: f64) -> (f64, f64, f64) {
    let t23030 = 2.0_f64 / 15.0_f64 * t173 * (t23004 + t23025) * t184 * t199;
    let t23032 = 4.0_f64 / 15.0_f64 * t18681 * t826;
    let t23033 = t22967 + t22971 - t22975 + t22978 + t22981 + t15144 + t15146 + 2.0_f64 * t15147 + t22983 + 2.0_f64 / 3.0_f64 * t19318 + 4.0_f64 / 3.0_f64 * t19320 + t23030 + t23032;
    (t23030, t23032, t23033)
}
