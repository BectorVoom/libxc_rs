//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1461;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta385<F: Float>(t232: F, t4119: F, t2645: F, t4181: F, t16891: F, t2647: F, t13242: F, t5591: F, t13228: F, t13351: F, t13222: F, t16839: F, t9627: F, t2632: F, t4233: F, t4180: F, t2639: F, t5619: F, t5614: F, t1484: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16914, t16918, t16924, t16927, t16928, t16932) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1461::<F>(t232, t4119, t2645, t4181, t16891, t2647, t13242, t5591, t13228, t13351, t13222, t16839, t9627);
        let (t16935, t16937, t16940, t16942, t16944) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1462::<F>(t2632, t4233, t4180, t4181, t2639, t5619, t5614, t1484, t4119);
    (t16914, t16918, t16924, t16927, t16928, t16932, t16935, t16937, t16940, t16942, t16944)
}
