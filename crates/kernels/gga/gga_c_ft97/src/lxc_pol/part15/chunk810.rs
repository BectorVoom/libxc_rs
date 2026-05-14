//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 810/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk810<F: Float>(t422: F, t4466: F, t4441: F, t528: F, t1736: F, t71: F, t39976: F, t4687: F, t135: F, t16891: F, t2057: F, t4698: F, t1636: F, t4669: F, t89: F, t4715: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61819 = t422 * t4466;
    let t61854 = t4441 * t528;
    let t61866 = t1736 * t4441;
    let t61889 = t71 * t4466;
    let t61965 = 0.59031789687271907073e-3 * t39976 * t4687;
    let t62087 = t16891 * t135;
    let t62090 = t2057 * t4698;
    let t62134 = t89 * t1636 * t4669;
    let t62246 = t89 * t1636 * t4715;
    (t61819, t61854, t61866, t61889, t61965, t62087, t62090, t62134, t62246)
}
