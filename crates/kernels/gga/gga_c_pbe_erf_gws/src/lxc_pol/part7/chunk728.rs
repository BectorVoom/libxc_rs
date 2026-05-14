//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 728/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk728<F: Float>(t2345: F, t6206: F, t875: F, t2119: F, t2387: F, t2124: F, t2145: F, t2150: F, t3074: F, t6252: F, t2112: F, t5: F, t337: F, t2121: F, t2365: F, t885: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6316 = t2345 * t6206 * t875;
    let t6319 = t2387 * t2119;
    let t6321 = t6319 * t2124 / 48.0;
    let t6322 = t2387 * t2145;
    let t6324 = t6322 * t2150 / 16.0;
    let t6325 = t3074 * t6252;
    let t6326 = t5 * t2112;
    let t6327 = t337 * t6326;
    let t6328 = t2121 * t6327;
    let t6330 = t6325 * t6328 / 32.0;
    let t6331 = t2365 * t885;
    (t6316, t6319, t6321, t6322, t6324, t6325, t6326, t6327, t6328, t6330, t6331)
}
