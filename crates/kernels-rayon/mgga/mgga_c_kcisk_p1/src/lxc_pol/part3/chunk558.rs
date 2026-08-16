//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 558/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk558(t4637: f64, t4638: f64, t4642: f64, t4646: f64, t4650: f64) -> f64 {
    let t4652 = t4637 + 2.0_f64 / 9.0_f64 * t4638 - 2.0_f64 / 9.0_f64 * t4642 + 2.0_f64 / 3.0_f64 * t4646 - t4650 / 3.0_f64;
    t4652
}
