//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 584/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk584<F: Float>(t364: F, t7773: F, t89: F, t1554: F, t375: F, t1642: F, t369: F, t1556: F, t21: F, t1586: F, t378: F, t1702: F, t52: F, t1614: F, t408: F, t1608: F, t373: F) -> (F, F, F, F, F, F, F) {
    let t7775 = t89 * t7773 * t364;
    let t7780 = t375 * t1554;
    let t7793 = t1642 * t369;
    let t7800 = 1.0 / t1556 / t21;
    let t7824 = t378 * t1586;
    let t7839 = t52 * t1702;
    let t7843 = t408 * t1614;
    let t7845 = t1608 * t7843 * t373;
    (t7775, t7780, t7793, t7800, t7824, t7839, t7845)
}
