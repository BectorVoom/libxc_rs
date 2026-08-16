//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1296/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1296(t13276: f64, t13282: f64, t13287: f64, t13289: f64, t13293: f64, t13297: f64, t13299: f64, t13302: f64, t13304: f64, t13307: f64, t13310: f64, t13315: f64, t13317: f64) -> f64 {
    let t15079 = t13276 + t13282 + t13287 + t13289 + t13293 - t13297 + t13299 + t13302 + t13304 + t13307 - t13310 + t13315 - t13317;
    t15079
}
