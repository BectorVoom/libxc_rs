//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 100/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk100(t250: f64, t252: f64, t461: f64, t453: f64, t456: f64, t459: f64) -> (f64, f64, f64, f64) {
    let t463 = t250 * t252 * t461;
    let t465 = 0.379785e1_f64 * t456 + 0.8969e0_f64 * t453 + 0.204775e0_f64 * t459 + 0.123235e0_f64 * t463;
    let t468 = 1.0_f64 + 0.16081824322151104822e2_f64 / t465;
    let t469 = f64::ln(t468);
    (t463, t465, t468, t469)
}
