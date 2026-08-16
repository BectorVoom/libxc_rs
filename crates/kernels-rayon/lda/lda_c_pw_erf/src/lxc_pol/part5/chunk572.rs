//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 572/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk572(t133: f64, t3227: f64, t153: f64, t274: f64, t2869: f64, t1125: f64, t678: f64, t1459: f64, t529: f64, t1283: f64, t518: f64) -> (f64, f64, f64, f64, f64) {
    let t3349 = t133 * t3227;
    let t3373 = 4.429070076315393_f64 * t153 * t2869 * t274;
    let t3378 = t153 * t1125 * t678;
    let t3402 = t1459 * t529;
    let t3416 = t1283 * t518;
    (t3349, t3373, t3378, t3402, t3416)
}
