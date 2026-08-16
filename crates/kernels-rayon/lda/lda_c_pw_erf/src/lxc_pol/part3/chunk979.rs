//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 979/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk979(t8278: f64, t8281: f64, t8286: f64, t11352: f64, t11353: f64, t11356: f64, t11357: f64, t11360: f64, t11362: f64, t11363: f64, t8260: f64, t8263: f64, t8266: f64, t8271: f64, t8274: f64, t8277: f64, t8285: f64, t8290: f64) -> (f64, f64, f64, f64) {
    let t11364 = 0.09759222794503372_f64 * t8278;
    let t11365 = 0.032530742648344574_f64 * t8281;
    let t11366 = 0.04879611397251686_f64 * t8286;
    let t11367 = -t11352 + t11353 - t11356 - t11357 + t8260 + t11360 + t11362 + t8263 - t8266 - t11363 + t8271 + t8274 - t8277 - t11364 + t11365 + t8285 + t11366 + t8290;
    (t11364, t11365, t11366, t11367)
}
