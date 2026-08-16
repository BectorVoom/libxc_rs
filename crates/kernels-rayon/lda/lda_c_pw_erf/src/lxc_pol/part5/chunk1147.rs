//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1147/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1147(t16239: f64, t16245: f64, t16253: f64, t16261: f64, t12083: f64, t16514: f64, t16516: f64, t16520: f64, t9437: f64, t16537: f64, t16600: f64, t14992: f64, t19134: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21112 = 32.0_f64 / 45.0_f64 * t16239;
    let t21113 = 16.0_f64 / 45.0_f64 * t16245;
    let t21114 = 16.0_f64 / 45.0_f64 * t16253;
    let t21115 = 8.0_f64 / 45.0_f64 * t16261;
    let t21116 = 16.0_f64 / 135.0_f64 * t12083;
    let t21118 = 16.0_f64 / 45.0_f64 * t16514;
    let t21119 = 32.0_f64 / 45.0_f64 * t16516;
    let t21120 = 16.0_f64 / 15.0_f64 * t16520;
    let t21121 = 32.0_f64 / 1215.0_f64 * t9437;
    let t21123 = 16.0_f64 / 45.0_f64 * t16537;
    let t21124 = 4.0_f64 / 45.0_f64 * t16600;
    let t21125 = t21112 + t21113 + t21114 - t21115 + t21116 - 2.0_f64 / 15.0_f64 * t19134 + t21118 - t21119 - t21120 + t21121 - 0.19947266666666666_f64 * t14992 - t21123 + t21124;
    (t21112, t21113, t21114, t21115, t21116, t21118, t21119, t21120, t21121, t21123, t21124, t21125)
}
