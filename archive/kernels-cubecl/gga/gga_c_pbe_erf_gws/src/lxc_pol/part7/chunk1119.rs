//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1119/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1119<F: Float>(t6729: F, t894: F, t2083: F, t2108: F, t825: F, t2169: F, t2200: F, t329: F, t2412: F, t2239: F, t4442: F, t4414: F, t6828: F) -> (F, F, F, F, F, F) {
    let t20081 = t6729 * t894;
    let t20085 = t2083 * t2108;
    let t20086 = t20085 * t825;
    let t20091 = t329 * t2200 * t2169;
    let t20092 = t20091 * t2412;
    let t20106 = t4442 * t2239;
    let t20108 = t4414 * t6828;
    (t20081, t20085, t20086, t20092, t20106, t20108)
}
