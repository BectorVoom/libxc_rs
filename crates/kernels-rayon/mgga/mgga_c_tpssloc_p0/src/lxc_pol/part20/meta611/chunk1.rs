//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2199/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2199(t3623: f64, t44927: f64, t11880: f64, t44690: f64, t11913: f64, t11931: f64, t225: f64, t11604: f64, t496: f64, t68: f64, t11601: f64, t11599: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45323 = t44927 * t3623;
    let t45326 = t44690 * t11880;
    let t45329 = t44690 * t11913;
    let t45345 = t11931 * t225;
    let t45349 = 1.0_f64 / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45355 = t11601 * t225;
    let t45375 = t11599 * t225;
    (t45323, t45326, t45329, t45345, t45350, t45355, t45375)
}
