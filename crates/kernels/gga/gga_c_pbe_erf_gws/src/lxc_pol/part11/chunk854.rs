//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 854/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk854<F: Float>(t329: F, t340: F, t6593: F, t20692: F, t825: F, t2200: F, t369: F, t2298: F, t332: F, t21637: F, t378: F, t838: F, t931: F, t2052: F, t4836: F, t4839: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21681 = t329 * t6593 * t340;
    let t21764 = t20692 * t825;
    let t21780 = t329 * t2200 * t369;
    let t21807 = t329 * t332 * t2298;
    let t21823 = 455.0 / 243.0 * t329 * t21637 * t378;
    let t21825 = t329 * t838 * t931;
    let t21884 = t2052 * t2052;
    let t21885 = 1.0 / t21884;
    let t21910 = 0.14035736153892489771e2 * t4836;
    let t21911 = 0.86748647062252193714e-1 * t4839;
    (t21681, t21764, t21780, t21807, t21823, t21825, t21885, t21910, t21911)
}
