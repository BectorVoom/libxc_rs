//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 180/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk180(t113: f64, t733: f64, t738: f64, t740: f64, t743: f64) -> f64 {
    let t745 = 0.59778596625315888114e-2_f64 * t113 - 0.17565e-2_f64 * t733 + 0.39625e-3_f64 * t738 - 0.1294884726949076719e-4_f64 * t740 + 0.1260328125e-5_f64 * t743;
    t745
}
