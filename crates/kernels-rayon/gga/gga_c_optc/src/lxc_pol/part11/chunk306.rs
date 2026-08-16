//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 306/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk306(t127: f64, t1271: f64, t5: f64, t675: f64, t116: f64, t1256: f64, t627: f64, t1273: f64, t696: f64, t1278: f64, t673: f64, t684: f64, t686: f64, t695: f64, t703: f64, t705: f64) -> (f64, f64, f64, f64) {
    let t1286 = t5 * t1271 * t127;
    let t1287 = t675 * t1286;
    let t1290 = t116 * t1256;
    let t1291 = t627 * t1290;
    let t1294 = t696 * t1273;
    let t1299 = -0.86931614897887578546e-1_f64 * t673 * t1287 - t684 - 0.17386322979577515709e0_f64 * t686 * t1291 - 0.15114211337509259186e-1_f64 * t695 * t1294 - t703 - 0.30228422675018518372e-1_f64 * t705 * t1278;
    (t1287, t1291, t1294, t1299)
}
