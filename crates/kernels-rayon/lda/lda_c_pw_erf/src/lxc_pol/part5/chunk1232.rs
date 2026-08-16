//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1232/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1232(t1325: f64, t1440: f64, t15595: f64, t784: f64, t518: f64, t7675: f64, t577: f64, t4753: f64, t7597: f64, t3416: f64, t1318: f64, t1466: f64, t17759: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t22189 = 4.0_f64 / 5.0_f64 * t1325 * t1440 * t15595 * t784;
    let t22190 = t7675 * t518;
    let t22192 = 4.0_f64 / 45.0_f64 * t22190 * t577;
    let t22194 = 4.0_f64 / 5.0_f64 * t4753 * t7597;
    let t22196 = 4.0_f64 / 5.0_f64 * t3416 * t7597;
    let t22200 = 4.0_f64 / 5.0_f64 * t1318 * t1466 * t17759 * t833;
    (t22189, t22192, t22194, t22196, t22200)
}
