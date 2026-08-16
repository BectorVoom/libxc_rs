//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1080/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1080(t11349: f64, t11360: f64, t20049: f64, t20054: f64, t20055: f64, t8221: f64, t8224: f64, t8238: f64, t8244: f64, t8248: f64, t8260: f64, t8263: f64, t8266: f64, t8271: f64, t8274: f64, t8277: f64) -> f64 {
    let t20191 = -t20049 - t11349 - t8221 + t8224 + t8238 - t8244 - t8248 + t8260 + t11360 + t20054 + t8263 - t8266 - t20055 + t8271 + t8274 - t8277;
    t20191
}
