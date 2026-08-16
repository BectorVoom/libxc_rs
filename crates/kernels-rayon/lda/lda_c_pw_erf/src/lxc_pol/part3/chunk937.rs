//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 937/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk937(t220: f64, t4567: f64, t211: f64, t1524: f64, t1529: f64, t3802: f64, t3850: f64, t519: f64, t1446: f64, t3803: f64, t197: f64, t3783: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10436 = t4567 * t220;
    let t10438 = 112.0_f64 / 1215.0_f64 * t211 * t10436;
    let t10439 = t1524 * t1529;
    let t10454 = t519 * t3802 * t3850;
    let t10456 = t1446 * t3803;
    let t10463 = t3783 * t197;
    (t10436, t10438, t10439, t10454, t10456, t10463)
}
