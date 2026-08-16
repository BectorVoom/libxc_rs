//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1249/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1249(t2072: f64, t22396: f64, t13930: f64, t22372: f64, t22374: f64, t22376: f64, t22378: f64, t22380: f64, t22382: f64, t22384: f64, t22386: f64, t22388: f64, t22391: f64, t22392: f64, t22394: f64) -> (f64, f64) {
    let t22398 = 8.0_f64 / 5.0_f64 * t22396 * t2072;
    let t22399 = t22372 - t22374 + t22376 - t22378 - t22380 - t22382 - t22384 - t22386 - t22388 - t13930 - t22391 - t22392 - t22394 - t22398;
    (t22398, t22399)
}
