//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 925/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk925(t13780: f64, t13794: f64, t13809: f64, t13811: f64, t13759: f64, t13775: f64, t13778: f64, t13783: f64, t13786: f64, t13789: f64, t13792: f64, t13798: f64, t13801: f64, t13804: f64, t13807: f64, t13814: f64, t13817: f64, t13820: f64, t13823: f64, t9699: f64) -> f64 {
    let t14336 = t13780 / 27.0_f64;
    let t14341 = 2.0_f64 / 81.0_f64 * t13794;
    let t14346 = t13809 / 27.0_f64;
    let t14347 = 2.0_f64 / 27.0_f64 * t13811;
    let t14352 = -2.0_f64 / 9.0_f64 * t13759 + t13775 / 18.0_f64 + t13778 / 27.0_f64 - t14336 + t13783 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t13786 + t13789 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t13792 + t14341 - t13798 / 27.0_f64 - 5.0_f64 / 81.0_f64 * t13801 + 4.0_f64 / 27.0_f64 * t13804 + t13807 / 18.0_f64 - t14346 - t9699 - t14347 - t13814 / 9.0_f64 - t13817 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t13820 - t13823 / 9.0_f64;
    t14352
}
