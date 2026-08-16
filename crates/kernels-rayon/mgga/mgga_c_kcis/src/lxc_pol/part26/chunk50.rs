//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 50/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk50(t104: f64, t111: f64, t113: f64, t120: f64, t89: f64) -> f64 {
    let t122 = -0.59778596625315888114e-2_f64 * t89 + 0.1317375e-2_f64 * t104 - 0.23775e-3_f64 * t111 + 0.64744236347453835951e-5_f64 * t113 - 0.540140625e-6_f64 * t120;
    t122
}
