//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1033/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1033(t2147: f64, t46976: f64, t1763: f64, t2084: f64, t27: f64, t7263: f64, t2191: f64, t9817: f64, t1986: f64, t6599: f64, t675: f64, t2868: f64, t40339: f64, t43375: f64, t46933: f64, t46938: f64, t46943: f64, t46948: f64, t46953: f64, t46958: f64, t46963: f64, t46969: f64, t46974: f64, t6344: f64, t668: f64, t72: f64, t8378: f64) -> f64 {
    let t46977 = t46976 * t2147;
    let t46981 = t7263 * t27 * t2084 * t1763;
    let t46985 = t2191 * t9817;
    let t46989 = t675 * t1986 * t6599;
    let t46991 = 0.23948483403727617128e0_f64 * t2868 * t8378 - 0.11971293719990017331e-4_f64 * t46933 + 0.35913881159970051993e-4_f64 * t46938 - 0.35913881159970051993e-4_f64 * t46943 - 0.11971293719990017331e-4_f64 * t46948 - 0.3192344991997337955e-4_f64 * t46953 - 0.1064114997332445985e-4_f64 * t46958 + 0.1064114997332445985e-4_f64 * t46963 - 0.85129199786595678796e-5_f64 * t46969 + 0.31923449919973379548e-4_f64 * t46974 - t43375 - 0.34093327067806677161e-2_f64 * t46977 - 0.18183107769496894486e-1_f64 * t46981 + t72 * t6344 * t668 - 0.12769379967989351819e-4_f64 * t46985 + 0.59590439850616975157e-4_f64 * t40339 - 0.12769379967989351819e-4_f64 * t46989;
    t46991
}
