//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1025/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1025<F: Float>(t2105: F, t814: F, t6587: F, t899: F, t912: F, t918: F, t6198: F, t6416: F, t6183: F, t6569: F, t6567: F, t2147: F, t337: F, t6340: F, t810: F, t6339: F) -> (F, F, F, F, F) {
    let t20640 = t2105 * t814;
    let t20646 = t899 * t912 * t6587;
    let t20647 = t20646 * t918;
    let t20649 = t6416 * t6198;
    let t20651 = t6183 * t6569;
    let t20652 = t6567 * t20651;
    let t20653 = 7.0 / 36.0 * t20652;
    let t20656 = t2147 * t337 * t6340 * t810;
    let t20658 = t6339 * t20656 / 4.0;
    (t20640, t20647, t20649, t20653, t20658)
}
