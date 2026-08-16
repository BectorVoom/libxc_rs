//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 934/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk934(t1124: f64, t265: f64, t266: f64, t3990: f64, t640: f64, t653: f64, t1125: f64, t252: f64, t254: f64, t1410: f64, t1433: f64, t1426: f64, t635: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11097 = 56.0_f64 / 1215.0_f64 * t265 * t266 * t1124;
    let t11098 = t640 * t3990;
    let t11101 = 32.0_f64 / 81.0_f64 * t653 * t3990;
    let t11104 = 56.0_f64 / 243.0_f64 * t252 * t254 * t1125;
    let t11153 = 4.0_f64 / 9.0_f64 * t1433 * t1410;
    let t11156 = 0.05402469135802469_f64 * t645 * t635 * t1426;
    (t11097, t11098, t11101, t11104, t11153, t11156)
}
