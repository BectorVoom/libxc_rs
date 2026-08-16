//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 847/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk847(t1125: f64, t153: f64, t865: f64, t1210: f64, t168: f64, t861: f64, t1891: f64, t474: f64, t156: f64, t3373: f64, t4092: f64, t4095: f64, t4096: f64, t4099: f64, t4101: f64, t4103: f64, t4106: f64, t4110: f64, t4113: f64, t5718: f64) -> f64 {
    let t5904 = t153 * t1125 * t865;
    let t5907 = t168 * t1210 * t861;
    let t5911 = 1.1389037339096726_f64 * t153 * t474 * t1891;
    let t5920 = -t3373 + 1.328721022894618_f64 * t5904 - 0.053059442957798957_f64 * t5907 - t5911 - 0.1675256410710088_f64 * t4092 - t4095 - 0.3350512821420176_f64 * t4096 - t4099 + 0.0837628205355044_f64 * t4101 + 0.3350512821420176_f64 * t4103 + t4106 - 0.0837628205355044_f64 * t4110 + t4113 + 0.42708890021612717_f64 * t153 * t156 * t5718;
    t5920
}
