//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 956/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk956(t180: f64, t9314: f64, t160: f64, t167: f64, t1071: f64, t253: f64, t2843: f64, t329: f64, t2822: f64, t2826: f64, t2831: f64, t2820: f64, t2840: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9315 = t180 * t9314;
    let t9323 = t167 * t160;
    let t9368 = 1.0_f64 / t253 / t1071;
    let t9372 = 1.0_f64 / t2843 / t329;
    let t9379 = t2822 * t2826;
    let t9383 = t2822 * t2831;
    let t9386 = t86 * t2820 * t2840;
    (t9315, t9323, t9368, t9372, t9379, t9383, t9386)
}
