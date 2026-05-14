//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 729/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk729<F: Float>(t2149: F, t6331: F, t2146: F, t346: F, t4395: F, t2382: F, t2124: F, t2119: F, t5: F, t745: F, t337: F, t2121: F, t810: F, t816: F, t2084: F, t3257: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6332 = t6331 * t2149;
    let t6333 = t2146 * t6332;
    let t6334 = 7.0 / 24.0 * t6333;
    let t6335 = t4395 * t346;
    let t6336 = t2382 * t6335;
    let t6338 = t6336 * t2124 / 32.0;
    let t6339 = t2382 * t2119;
    let t6340 = t5 * t745;
    let t6341 = t337 * t6340;
    let t6342 = t2121 * t6341;
    let t6344 = t6339 * t6342 / 32.0;
    let t6345 = t816 * t810;
    let t6347 = t3257 * t2084 * t6345;
    (t6332, t6334, t6335, t6336, t6338, t6339, t6340, t6341, t6342, t6344, t6345, t6347)
}
