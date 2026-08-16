//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 686/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk686(t2095: f64, t831: f64, t1420: f64, t2481: f64, t1426: f64, t2480: f64, t439: f64, t444: f64, t5961: f64, t442: f64, t2485: f64, t2484: f64, t3279: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6238 = t831 * t2095 / 15.0_f64;
    let t6240 = t1420 * t2481 / 45.0_f64;
    let t6241 = t1426 * t2480;
    let t6243 = t439 * t6241 / 45.0_f64;
    let t6244 = t444 * t5961;
    let t6245 = t442 * t6244;
    let t6247 = t439 * t6245 / 45.0_f64;
    let t6249 = t1420 * t2485 / 27.0_f64;
    let t6250 = t3279 * t2484;
    (t6238, t6240, t6241, t6243, t6244, t6245, t6247, t6249, t6250)
}
