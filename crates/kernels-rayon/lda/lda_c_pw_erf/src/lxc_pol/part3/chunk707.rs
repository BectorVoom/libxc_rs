//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 707/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk707(t3437: f64, t548: f64, t1529: f64, t822: f64, t1982: f64, t515: f64, t1960: f64, t568: f64, t3380: f64, t3385: f64, t3388: f64, t3391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4464 = 4.0_f64 / 15.0_f64 * t548 * t3437;
    let t4465 = t822 * t1529;
    let t4466 = 4.0_f64 / 135.0_f64 * t4465;
    let t4468 = 8.0_f64 / 45.0_f64 * t1982 * t515;
    let t4470 = 8.0_f64 / 45.0_f64 * t1960 * t568;
    let t4471 = 16.0_f64 / 45.0_f64 * t3380;
    let t4472 = 8.0_f64 / 45.0_f64 * t3385;
    let t4473 = 8.0_f64 / 45.0_f64 * t3388;
    let t4474 = 8.0_f64 / 45.0_f64 * t3391;
    (t4464, t4466, t4468, t4470, t4471, t4472, t4473, t4474)
}
