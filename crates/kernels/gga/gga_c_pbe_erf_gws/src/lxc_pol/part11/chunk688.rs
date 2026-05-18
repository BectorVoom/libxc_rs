//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 688/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk688<F: Float>(t3205: F, t329: F, t838: F, t1164: F, t2242: F, t3133: F, t6183: F, t3179: F, t6331: F, t1133: F, t2157: F, t332: F, t6238: F, t863: F) -> (F, F, F, F, F, F) {
    let t8801 = t329 * t838 * t3205;
    let t8818 = t2242 * t1164;
    let t8824 = t6183 * t3133;
    let t8833 = t6331 * t3179;
    let t8884 = t1133 * t2157;
    let t8903 = t863 * t6238 * t332;
    (t8801, t8818, t8824, t8833, t8884, t8903)
}
