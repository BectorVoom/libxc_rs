//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 466/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk466(t2658: f64, t2347: f64, t295: f64, t2349: f64, t2345: f64, t89: f64, t683: f64, t798: f64) -> (f64, f64, f64, f64) {
    let t2659 = t2658 / 9.0_f64;
    let t2660 = t295 * t2347;
    let t2661 = t2660 * t2349;
    let t2663 = t89 * t2345 * t2661;
    let t2665 = t683 * t798;
    (t2659, t2661, t2663, t2665)
}
