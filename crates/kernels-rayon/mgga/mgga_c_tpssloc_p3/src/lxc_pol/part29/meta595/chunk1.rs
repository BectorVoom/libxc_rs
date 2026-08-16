//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2025/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2025(t22988: f64, t23110: f64, t81651: f64, t22893: f64, t23154: f64, t23164: f64, t234: f64, t2710: f64, t23176: f64, t23185: f64, t131: f64, t2587: f64, t81142: f64) -> (f64, f64, f64, f64, f64) {
    let t81653 = t81651 * t23110 * t22988;
    let t81656 = t23164 * t22893 * t23154;
    let t81658 = t234 * t2710;
    let t81670 = t23185 * t23110 * t23176;
    let t81686 = t81142 * t2587 * t131;
    (t81653, t81656, t81658, t81670, t81686)
}
