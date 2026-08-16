//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1063/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1063(t2033: f64, t3925: f64, t10159: f64, t10164: f64, t10169: f64, t10174: f64, t2024: f64, t2027: f64, t684: f64, t687: f64, t8224: f64, t8450: f64, t8452: f64, t8454: f64, t8462: f64, t8476: f64, t8479: f64, t8491: f64, t8501: f64) -> (f64, f64) {
    let t10176 = t2033 * t3925;
    let t10180 = t8224 / 48.0_f64 - t8450 - t8452 + t8454 / 48.0_f64 - t8462 - t8476 + t8479 / 48.0_f64 - t8491 - t8501 - t684 * t687 * t10159 / 64.0_f64 - t684 * t687 * t10164 / 32.0_f64 - t684 * t687 * t10169 / 64.0_f64 - t10174 / 144.0_f64 - t2024 * t2027 * t10176 / 48.0_f64;
    (t10176, t10180)
}
