//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 717/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk717(t1319: f64, t6413: f64, t571: f64, t2325: f64, t3518: f64, t348: f64, t5250: f64, t519: f64, t1966: f64, t34: f64, t5256: f64, t2471: f64, t504: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6414 = t1319 * t6413;
    let t6416 = 8.0_f64 / 15.0_f64 * t571 * t6414;
    let t6417 = t3518 * t2325;
    let t6418 = t6417 * t348;
    let t6419 = t5250 * t6418;
    let t6421 = 32.0_f64 / 81.0_f64 * t519 * t6419;
    let t6422 = t1966 * t34;
    let t6423 = t5256 * t6422;
    let t6425 = 16.0_f64 / 27.0_f64 * t519 * t6423;
    let t6426 = t2471 * t504;
    (t6414, t6416, t6417, t6418, t6419, t6421, t6422, t6423, t6425, t6426)
}
