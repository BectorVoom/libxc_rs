//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1243/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1243<F: Float>(t2721: F, t7983: F, t8152: F, t8044: F, t2670: F, t7212: F, t3917: F, t7494: F, t10838: F, t8164: F, t3884: F, t7452: F) -> (F, F, F, F, F) {
    let t25657 = t2721 * t8152 * t7983;
    let t25660 = t2721 * t8152 * t8044;
    let t25662 = t7212 * t2670;
    let t25664 = t3917 * t25662 * t7494;
    let t25667 = t2721 * t10838 * t8164;
    let t25670 = t3884 * t25662 * t7452;
    (t25657, t25660, t25664, t25667, t25670)
}
