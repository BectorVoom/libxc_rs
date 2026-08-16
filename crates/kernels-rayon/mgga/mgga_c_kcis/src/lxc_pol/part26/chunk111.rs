//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 111/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk111(t453: f64, t456: f64) -> (f64, f64, f64, f64) {
    let t513 = 0.107924e1_f64 + 0.3964e-1_f64 * t456 + 0.123825e-1_f64 * t453;
    let t516 = 1.0_f64 + t456 * t513 / 2.0_f64;
    let t517 = t516 * t516;
    let t518 = 1.0_f64 / t517;
    (t513, t516, t517, t518)
}
