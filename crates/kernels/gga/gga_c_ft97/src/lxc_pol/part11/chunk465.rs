//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 465/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk465<F: Float>(t2681: F, t2682: F, t27: F, t89: F, t811: F, t284: F, t291: F, t287: F, t800: F, t816: F, t820: F, t194: F, t272: F, t2380: F, t274: F, t2417: F, t801: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2683 = t2681 * t2682;
    let t2685 = t89 * t27 * t2683;
    let t2687 = t811 * t811;
    let t2688 = t2687 * t284;
    let t2689 = t2688 * t291;
    let t2691 = t800 * t287;
    let t2692 = t816 * t811;
    let t2693 = t2692 * t820;
    let t2697 = 1.0 / t272 / t194;
    let t2698 = t2697 * t2380;
    let t2699 = t2698 * t274;
    let t2701 = t801 * t2417;
    (t2683, t2685, t2688, t2689, t2691, t2692, t2693, t2697, t2698, t2699, t2701)
}
