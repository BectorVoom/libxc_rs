//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 443/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk443(t2023: f64, t294: f64, t46: f64, t2020: f64, t133: f64) -> (f64, f64, f64, f64) {
    let t2024 = t294 * t2023;
    let t2025 = t2024 * t46;
    let t2026 = t2020 * t2025;
    let t2029 = t133 * t133;
    (t2024, t2025, t2026, t2029)
}
