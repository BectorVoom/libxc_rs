//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 774/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk774<F: Float>(t20696: F, t2476: F, t9438: F, t20561: F, t2487: F, t12444: F, t2464: F, t587: F, t6125: F, t883: F, t286: F, t39622: F, t708: F) -> (F, F, F, F, F) {
    let t40449 = t2476 * t9438 * t20696;
    let t40452 = t2487 * t9438 * t20561;
    let t40517 = t587 * t2464 * t12444;
    let t40594 = t883 * t6125;
    let t40612 = t39622 * t286 * t708;
    (t40449, t40452, t40517, t40594, t40612)
}
