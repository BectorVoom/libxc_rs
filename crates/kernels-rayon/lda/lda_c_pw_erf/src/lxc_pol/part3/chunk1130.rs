//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1130/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1130(t11766: f64, t1325: f64, t5256: f64, t1446: f64, t5421: f64, t13201: f64, t13206: f64, t13208: f64, t13210: f64, t13212: f64, t13214: f64, t13216: f64, t13221: f64, t13223: f64, t13225: f64, t13229: f64) -> (f64, f64, f64) {
    let t13232 = 8.0_f64 / 9.0_f64 * t1325 * t5256 * t11766;
    let t13233 = t1446 * t5421;
    let t13234 = 16.0_f64 / 45.0_f64 * t13233;
    let t13235 = t13201 + t13206 + t13208 + t13210 + t13212 - t13214 - t13216 - t13221 + t13223 + t13225 + t13229 + t13232 - t13234;
    (t13232, t13234, t13235)
}
