//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 508/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk508(t684: f64, t687: f64, t686: f64, t81: f64, t60: f64, t705: f64, t78: f64, t159: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2381 = t684 * t687;
    let t2385 = 1.0_f64 / t686 / t81;
    let t2386 = t60 * t2385;
    let t2387 = t705 * t705;
    let t2388 = t78 * t78;
    let t2389 = 1.0_f64 / t2388;
    let t2390 = t2387 * t2389;
    let t2394 = 1.0_f64 / t9 / t159;
    (t2381, t2385, t2386, t2387, t2388, t2389, t2390, t2394)
}
