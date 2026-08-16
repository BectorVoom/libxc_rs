//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1005/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1005<F: Float>(t2147: F, t46976: F, t1763: F, t2084: F, t27: F, t7263: F, t2191: F, t9817: F, t1986: F, t6599: F, t675: F, t2310: F, t9087: F) -> (F, F, F, F, F) {
    let t46977 = t46976 * t2147;
    let t46981 = t7263 * t27 * t2084 * t1763;
    let t46985 = t2191 * t9817;
    let t46989 = t675 * t1986 * t6599;
    let t46992 = t9087 * t2310;
    (t46977, t46981, t46985, t46989, t46992)
}
