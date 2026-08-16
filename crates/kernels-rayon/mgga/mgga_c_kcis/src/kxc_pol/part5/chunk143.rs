//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 143/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk143(t362: f64, t413: f64, t430: f64, t378: f64, t390: f64) -> (f64, f64, f64) {
    let t433 = t413 * t430 + 0.17411041666666666666e-2_f64 * t362;
    let t436 = 1.0_f64 + 0.9375e-1_f64 * t378 - 0.101171875e-1_f64 * t390;
    let t437 = 1.0_f64 / t436;
    (t433, t436, t437)
}
