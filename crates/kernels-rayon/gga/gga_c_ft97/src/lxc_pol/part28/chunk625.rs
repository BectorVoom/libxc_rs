//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 625/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk625(t26016: f64, t469: f64, t28: f64, t5665: f64, t1317: f64, t376: f64, t6504: f64, t6496: f64, t25846: f64, t370: f64, t27: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t26017 = t469 * t26016;
    let t26019 = t5665 * t28 * t26017;
    let t26022 = t1317 * t376 * t6504;
    let t26025 = t5665 * t376 * t6496;
    let t26027 = t370 * t25846;
    let t26029 = t89 * t27 * t26027;
    (t26019, t26022, t26025, t26027, t26029)
}
