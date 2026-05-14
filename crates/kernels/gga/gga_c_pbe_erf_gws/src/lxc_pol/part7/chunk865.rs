//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 865/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk865<F: Float>(t17609: F, t16563: F, t7062: F, t7069: F, t5038: F, t5211: F, t617: F, t7483: F, t4892: F, t610: F, t7514: F, t1406: F, t1828: F, t5218: F, t5219: F, t108: F, t1878: F, t267: F) -> (F, F, F, F, F, F) {
    let t17610 = 32.0 / 15.0 * t17609;
    let t17613 = 16.0 / 9.0 * t7062 * t7069 * t16563;
    let t17617 = 64.0 / 15.0 * t5211 * t7483 * t617 * t5038;
    let t17621 = 32.0 / 15.0 * t7062 * t7514 * t610 * t4892;
    let t17625 = 32.0 / 15.0 * t5218 * t5219 * t1406 * t1828;
    let t17627 = t1878 * t108 * t267;
    (t17610, t17613, t17617, t17621, t17625, t17627)
}
