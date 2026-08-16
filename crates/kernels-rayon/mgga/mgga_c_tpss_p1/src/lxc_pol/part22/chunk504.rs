//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 504/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk504(t234: f64, t64: f64, t2004: f64, t2010: f64, t2013: f64, t2017: f64, t2020: f64, t44: f64, t49: f64, t56: f64, t589: f64, t592: f64) -> (f64, f64, f64) {
    let t2023 = t64 * t234;
    let t2024 = 88.0_f64 / 9.0_f64 * t2023;
    let t2025 = 88.0_f64 / 9.0_f64 * t2004 * t49 - 40.0_f64 / 9.0_f64 * t589 * t592 + 5.0_f64 / 18.0_f64 * t44 * t2010 + 5.0_f64 / 6.0_f64 * t44 * t2013 + 5.0_f64 / 18.0_f64 * t56 * t2017 - 5.0_f64 / 6.0_f64 * t56 * t2020 - t2024;
    (t2023, t2024, t2025)
}
