//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1113/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1113(t13205: f64, t13207: f64, t13210: f64, t13212: f64, t13214: f64, t13216: f64, t13219: f64, t13221: f64, t13223: f64, t13225: f64, t13226: f64, t13227: f64) -> f64 {
    let t13228 = t13205 + t13207 + t13210 + t13212 + t13214 + t13216 - t13219 + t13221 - t13223 - t13225 - t13226 + t13227;
    t13228
}
