//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 928/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk928<F: Float>(t103: F, t193: F, t197: F, t2078: F, t102: F, t652: F, t2268: F, t47: F, t34: F, t543: F, t2854: F, t52: F, t538: F, t6325: F, t88: F, t1859: F) -> (F, F, F, F, F, F, F) {
    let t21988 = 261800.0 / 729.0 * t193 * t2078 * t103 * t197;
    let t21989 = t652 * t102;
    let t22014 = 1.0 / t47 / t2268;
    let t22026 = t34 * t543;
    let t22034 = 1.0 / t52 / t2854;
    let t22073 = t538 * t6325 * t88;
    let t22074 = 1920.0 * t22073;
    let t22075 = t1859 * t1859;
    (t21988, t21989, t22014, t22026, t22034, t22074, t22075)
}
