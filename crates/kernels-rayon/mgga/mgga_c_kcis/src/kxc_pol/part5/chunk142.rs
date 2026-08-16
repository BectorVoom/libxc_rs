//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 142/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk142(t209: f64, t417: f64, t421: f64, t416: f64) -> (f64, f64, f64, f64) {
    let t423 = t209 * t417 * t421;
    let t426 = 1.0_f64 + t416 * t423 / 192.0_f64;
    let t427 = f64::ln(t426);
    let t429 = 1.0_f64 + 0.66725e-1_f64 * t427;
    let t430 = 1.0_f64 / t429;
    (t423, t426, t429, t430)
}
