//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1228/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1228(t14449: f64, t14439: f64, t14440: f64, t14441: f64, t14442: f64, t14444: f64, t14446: f64, t14448: f64, t8527: f64, t8533: f64, t8536: f64, t8539: f64, t8542: f64, t8716: f64, t8733: f64, t8737: f64, t8740: f64) -> (f64, f64) {
    let t14450 = 0.0005696928233656539_f64 * t14449;
    let t14451 = -t14439 + t14440 + t8527 + t14441 + t8533 - t8536 + t8539 - t8542 + t14442 + t14444 - t14446 + t14448 - t14450 + t8733 - t8716 - t8737 + t8740;
    (t14450, t14451)
}
