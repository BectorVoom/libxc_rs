//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 452/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk452(t2469: f64, t766: f64, t242: f64, t250: f64, t251: f64, t747: f64, t91: f64, t1771: f64, t249: f64, t1775: f64, t740: f64, t458: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2470 = t2469 * t766;
    let t2471 = t242 * t2470;
    let t2475 = 1.0_f64 / t251 / t250;
    let t2476 = t747 * t747;
    let t2478 = t91 * t2475 * t2476;
    let t2481 = 4.0_f64 / 9.0_f64 * t1771 * t249;
    let t2482 = t1775 * t740;
    let t2484 = t458 * t745;
    (t2470, t2471, t2475, t2476, t2478, t2481, t2482, t2484)
}
