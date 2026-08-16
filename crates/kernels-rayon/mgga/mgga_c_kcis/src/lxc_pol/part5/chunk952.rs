//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 952/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk952(t180: f64, t9314: f64, t160: f64, t167: f64, t1071: f64, t253: f64, t2843: f64, t329: f64, t2820: f64, t2840: f64, t86: f64, t3225: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9315 = t180 * t9314;
    let t9323 = t167 * t160;
    let t9368 = 1.0_f64 / t253 / t1071;
    let t9372 = 1.0_f64 / t2843 / t329;
    let t9386 = t86 * t2820 * t2840;
    let t9409 = t3225 * sigma0;
    (t9315, t9323, t9368, t9372, t9386, t9409)
}
