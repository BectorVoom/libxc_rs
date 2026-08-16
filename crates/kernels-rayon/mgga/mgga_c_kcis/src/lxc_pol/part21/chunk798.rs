//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 798/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk798(t9296: f64, t9311: f64, t160: f64, t167: f64, t1071: f64, t253: f64, t1017: f64, t86: f64, t2843: f64, t329: f64, t2822: f64, t2826: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9312 = t9296 + t9311;
    let t9323 = t167 * t160;
    let t9368 = 1.0_f64 / t253 / t1071;
    let t9370 = t86 * t1017 * t9368;
    let t9372 = 1.0_f64 / t2843 / t329;
    let t9379 = t2822 * t2826;
    (t9312, t9323, t9368, t9370, t9372, t9379)
}
