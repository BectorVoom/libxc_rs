//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1064/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1064(t12464: f64, t12427: f64, t12432: f64, t12435: f64, t12438: f64, t12442: f64, t12444: f64, t12449: f64, t12453: f64, t12456: f64, t12459: f64, t12461: f64, t12463: f64) -> (f64, f64) {
    let t12465 = 32.0_f64 / 27.0_f64 * t12464;
    let t12466 = t12427 + t12432 + t12435 + t12438 + t12442 + t12444 + t12449 + t12453 + t12456 + t12459 - t12461 - t12463 + t12465;
    (t12465, t12466)
}
