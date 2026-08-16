//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 426/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk426(t2372: f64, t2373: f64, t27: f64, t89: f64, t196: f64, t122: f64) -> (f64, f64, f64, f64) {
    let t2374 = t2372 * t2373;
    let t2376 = t89 * t27 * t2374;
    let t2378 = 1.0_f64 / t196;
    let t2379 = t122 * t2378;
    (t2374, t2376, t2378, t2379)
}
