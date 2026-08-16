//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2093/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2093(t374: f64, t485: f64, t486: f64, t9697: f64, t11778: f64, t121: f64, t1229: f64, t204: f64, t1090: f64, t1227: f64, t248: f64, t11880: f64, t44690: f64) -> (f64, f64, f64, f64, f64) {
    let t45250 = 7.0_f64 / 31104.0_f64 * t485 * t374 * t9697 * t486;
    let t45268 = t121 * t11778;
    let t45293 = t204 * t1229;
    let t45296 = t1227 * t248 * t45293 * t1090;
    let t45326 = t44690 * t11880;
    (t45250, t45268, t45293, t45296, t45326)
}
