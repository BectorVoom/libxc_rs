//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 505/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk505(t2480: f64, t548: f64, t1268: f64, t2429: f64, t2433: f64, t538: f64, t2437: f64, t1240: f64, t1263: f64, t1964: f64, t2087: f64, t2431: f64, t2435: f64, t2439: f64, t25: f64) -> (f64, f64, f64, f64, f64) {
    let t2482 = 4.0_f64 / 15.0_f64 * t548 * t2480;
    let t2488 = t1268 * t2429;
    let t2491 = t538 * t2433;
    let t2494 = t538 * t2437;
    let t2497 = t1240 + 0.023994444444444443_f64 * t1964 - 0.023994444444444443_f64 * t2431 + 0.07198333333333333_f64 * t2435 - 0.035991666666666665_f64 * t2439 + t1263 + 0.008888888888888889_f64 * t2087 - 0.0022222222222222222_f64 * t25 * t2488 + 0.013333333333333334_f64 * t25 * t2491 - 0.006666666666666667_f64 * t25 * t2494;
    (t2482, t2488, t2491, t2494, t2497)
}
