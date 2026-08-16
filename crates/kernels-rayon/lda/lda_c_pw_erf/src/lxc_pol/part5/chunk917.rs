//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 917/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk917(t1159: f64, t603: f64, t1634: f64, t695: f64, t2070: f64, t493: f64, t495: f64, t4039: f64, t511: f64, t220: f64, t4567: f64, t211: f64) -> (f64, f64, f64, f64, f64) {
    let t10414 = t1159 * t603;
    let t10417 = 0.004413481481481482_f64 * t695 * t1634;
    let t10419 = t493 * t2070 * t495;
    let t10427 = t511 * t4039;
    let t10436 = t4567 * t220;
    let t10438 = 112.0_f64 / 1215.0_f64 * t211 * t10436;
    (t10414, t10417, t10419, t10427, t10438)
}
