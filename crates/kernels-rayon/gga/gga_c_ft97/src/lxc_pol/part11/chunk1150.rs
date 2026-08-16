//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1150/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1150(t10681: f64, t1882: f64, t10467: f64, t8392: f64, t10482: f64, t10478: f64, t863: f64, t10548: f64, t10769: f64, t10505: f64, t2360: f64, t2842: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44160 = t1882 * t10681;
    let t44174 = t8392 * t10467;
    let t44176 = t8392 * t10482;
    let t44178 = t10478 * t863;
    let t44190 = t1882 * t10548;
    let t44195 = t1882 * t10769;
    let t44202 = t8392 * t10505;
    let t44204 = t2842 * t2360;
    (t44160, t44174, t44176, t44178, t44190, t44195, t44202, t44204)
}
