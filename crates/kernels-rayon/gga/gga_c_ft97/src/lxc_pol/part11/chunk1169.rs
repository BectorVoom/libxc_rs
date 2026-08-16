//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1169/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1169(t43506: f64, t43519: f64, t43534: f64, t43537: f64, t43933: f64, t43936: f64, t43503: f64, t43511: f64, t43516: f64, t43522: f64, t43528: f64, t43531: f64, t43930: f64, t43940: f64) -> f64 {
    let t44769 = 4.0_f64 / 27.0_f64 * t43506;
    let t44771 = 8.0_f64 / 9.0_f64 * t43519;
    let t44775 = 8.0_f64 / 81.0_f64 * t43534;
    let t44776 = 140.0_f64 / 243.0_f64 * t43537;
    let t44778 = 56.0_f64 / 81.0_f64 * t43933;
    let t44779 = 4.0_f64 / 9.0_f64 * t43936;
    let t44781 = -6.0_f64 * t43503 - t44769 - 4.0_f64 / 3.0_f64 * t43511 + t43516 + t44771 + 4.0_f64 / 3.0_f64 * t43522 + 4.0_f64 * t43528 + 2.0_f64 / 9.0_f64 * t43531 - t44775 + t44776 - t43930 / 6.0_f64 + t44778 - t44779 + 4.0_f64 / 9.0_f64 * t43940;
    t44781
}
