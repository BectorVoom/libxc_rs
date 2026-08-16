//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1142/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1142(t13375: f64, t1620: f64, t838: f64, t1931: f64, t610: f64, t230: f64, t4714: f64, t4521: f64, t833: f64, t3610: f64, t4506: f64, t211: f64, t4567: f64, t4575: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13376 = 8.0_f64 / 27.0_f64 * t13375;
    let t13377 = t838 * t1620;
    let t13379 = t1931 * t610;
    let t13380 = 8.0_f64 * t13379;
    let t13381 = t4714 * t230;
    let t13384 = t4521 * t833;
    let t13387 = 4.0_f64 / 9.0_f64 * t4506 * t13384 * t3610;
    let t13389 = t211 * t4567 * t4575;
    (t13376, t13377, t13380, t13381, t13384, t13387, t13389)
}
