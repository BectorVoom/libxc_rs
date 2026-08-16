//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1147/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1147(t43537: f64, t43511: f64, t43516: f64, t43519: f64, t43522: f64, t43528: f64, t43531: f64, t43534: f64, t43541: f64, t43551: f64, t43926: f64, t43930: f64, t43933: f64, t43936: f64, t43940: f64) -> f64 {
    let t44121 = 280.0_f64 / 81.0_f64 * t43537;
    let t44128 = -8.0_f64 * t43511 + 6.0_f64 * t43516 + 16.0_f64 / 3.0_f64 * t43519 + 8.0_f64 * t43522 + 24.0_f64 * t43528 + 4.0_f64 / 3.0_f64 * t43531 - 16.0_f64 / 27.0_f64 * t43534 + t44121 - 3.0_f64 / 4.0_f64 * t43541 - 15.0_f64 / 16.0_f64 * t43551 + t43926 / 2.0_f64 - t43930 + 112.0_f64 / 27.0_f64 * t43933 - 8.0_f64 / 3.0_f64 * t43936 + 8.0_f64 / 3.0_f64 * t43940;
    t44128
}
