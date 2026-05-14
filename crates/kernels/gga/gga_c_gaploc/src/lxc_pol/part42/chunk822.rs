//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 822/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk822<F: Float>(t1377: F, t14348: F, t14266: F, t158: F, t203: F, t550: F, t123: F, t1358: F, t1365: F, t44258: F, t44262: F, t44263: F, t44264: F, t44267: F, t44278: F, t44281: F, t44284: F, t44288: F, t44292: F, t44293: F, t44298: F, t46859: F, t46862: F, t46865: F, t488: F) -> (F, F, F, F, F) {
    let t49820 = t1377 * t14348;
    let t49821 = t158 * t14266;
    let t49826 = t203 * t14266;
    let t49827 = t550 * t49826;
    let t49834 = -t44258 + t44262 + t44263 - t44264 - t44267 - t44278 + t44281 - t44284 + t44288 - 0.31616674039640166221e-2 * t1358 * t49821 * t123 * t488 + 0.31616674039640166221e-2 * t1358 * t1365 * t49827 - t44292 - t44293 + 0.47425011059460249332e-2 * t46859 - 0.142275033178380748e-1 * t46862 + 0.94850022118920498664e-2 * t46865 - t44298;
    (t49820, t49821, t49826, t49827, t49834)
}
