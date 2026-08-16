//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 601/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk601(t148: f64, t2929: f64, t1159: f64, t242: f64, t632: f64, t695: f64, t1198: f64, t1143: f64, t458: f64, t1155: f64, t285: f64, t477: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4095 = 0.0837628205355044_f64 * t148 * t2929;
    let t4096 = t1159 * t242;
    let t4099 = 0.5025769232130264_f64 * t695 * t632;
    let t4103 = t1198 * t632;
    let t4106 = 0.2512884616065132_f64 * t458 * t1143;
    let t4113 = 0.5025769232130264_f64 * t1155 * t242;
    let t4125 = t1159 * t477 * t285;
    (t4095, t4096, t4099, t4103, t4106, t4113, t4125)
}
