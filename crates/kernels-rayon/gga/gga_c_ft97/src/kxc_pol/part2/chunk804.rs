//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 804/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk804(t1882: f64, t3485: f64, t3408: f64, t558: f64, t167: f64, t2185: f64, t609: f64, t574: f64, t605: f64, t3450: f64, t616: f64, t2142: f64, t3455: f64) -> (f64, f64, f64, f64, f64) {
    let t12644 = 4.0_f64 / 9.0_f64 * t1882 * t3485;
    let t12645 = t3408 * t558;
    let t12647 = t2185 * t167 * t12645;
    let t12650 = t3408 * t609;
    let t12652 = t574 * t605 * t12650;
    let t12656 = t2185 * t616 * t3450;
    let t12660 = t574 * t2142 * t3455;
    (t12644, t12647, t12652, t12656, t12660)
}
