//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1293/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1293(t153: f64, t474: f64, t7387: f64, t11219: f64, t11222: f64, t11229: f64, t11232: f64, t11233: f64, t11236: f64, t14933: f64, t14935: f64, t14942: f64, t14943: f64, t14948: f64, t14950: f64, t14957: f64, t14958: f64, t14961: f64, t14965: f64, t19385: f64, t19388: f64, t19397: f64) -> f64 {
    let t23102 = t153 * t474 * t7387;
    let t23115 = -0.5694518669548363_f64 * t23102 + t14933 - 13.28721022894618_f64 * t14935 - t14942 - 1.5077307696390791_f64 * t14943 + t14948 + 0.7538653848195396_f64 * t14950 - t14957 - 0.2512884616065132_f64 * t14958 - t14961 - 0.2512884616065132_f64 * t19385 - 0.2512884616065132_f64 * t19388 + 1.5077307696390791_f64 * t14965 - 0.5025769232130264_f64 * t11219 - t11222 - 0.0837628205355044_f64 * t11229 - t11232 + 0.2512884616065132_f64 * t11233 + t11236 - 0.5025769232130264_f64 * t19397;
    t23115
}
