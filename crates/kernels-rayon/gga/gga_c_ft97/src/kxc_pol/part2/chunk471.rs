//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 471/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk471(t2681: f64, t2682: f64, t27: f64, t89: f64, t811: f64, t284: f64, t291: f64, t287: f64, t800: f64, t816: f64, t820: f64, t194: f64, t272: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2683 = t2681 * t2682;
    let t2685 = t89 * t27 * t2683;
    let t2687 = t811 * t811;
    let t2688 = t2687 * t284;
    let t2689 = t2688 * t291;
    let t2691 = t800 * t287;
    let t2692 = t816 * t811;
    let t2693 = t2692 * t820;
    let t2697 = 1.0_f64 / t272 / t194;
    (t2683, t2685, t2687, t2688, t2689, t2691, t2693, t2697)
}
