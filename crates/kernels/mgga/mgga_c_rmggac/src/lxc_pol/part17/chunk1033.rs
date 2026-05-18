//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1033/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1033<F: Float>(t2147: F, t46976: F, t1763: F, t2084: F, t27: F, t7263: F, t2191: F, t9817: F, t1986: F, t6599: F, t675: F, t2868: F, t40339: F, t43375: F, t46933: F, t46938: F, t46943: F, t46948: F, t46953: F, t46958: F, t46963: F, t46969: F, t46974: F, t6344: F, t668: F, t72: F, t8378: F) -> F {
    let t46977 = t46976 * t2147;
    let t46981 = t7263 * t27 * t2084 * t1763;
    let t46985 = t2191 * t9817;
    let t46989 = t675 * t1986 * t6599;
    let t46991 = F::new(0.23948483403727617128e0) * t2868 * t8378 - F::new(0.11971293719990017331e-4) * t46933 + F::new(0.35913881159970051993e-4) * t46938 - F::new(0.35913881159970051993e-4) * t46943 - F::new(0.11971293719990017331e-4) * t46948 - F::new(0.3192344991997337955e-4) * t46953 - F::new(0.1064114997332445985e-4) * t46958 + F::new(0.1064114997332445985e-4) * t46963 - F::new(0.85129199786595678796e-5) * t46969 + F::new(0.31923449919973379548e-4) * t46974 - t43375 - F::new(0.34093327067806677161e-2) * t46977 - F::new(0.18183107769496894486e-1) * t46981 + t72 * t6344 * t668 - F::new(0.12769379967989351819e-4) * t46985 + F::new(0.59590439850616975157e-4) * t40339 - F::new(0.12769379967989351819e-4) * t46989;
    t46991
}
