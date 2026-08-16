//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 644/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk644(t634: f64, t8640: f64, t2253: f64, t2277: f64, t2261: f64, t2284: f64, t422: f64, t639: f64, t2252: f64, t41: f64, t70: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8641 = t8640 * t634;
    let t8643 = t2253 * t2277;
    let t8645 = t2253 * t2261;
    let t8647 = t2253 * t2284;
    let t8654 = t422 * t639;
    let t8675 = t41 * t2252 * t70;
    (t8641, t8643, t8645, t8647, t8654, t8675)
}
