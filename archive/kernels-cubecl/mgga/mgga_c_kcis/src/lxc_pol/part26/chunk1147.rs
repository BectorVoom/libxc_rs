//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1147/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1147<F: Float>(t7091: F, t7899: F, t6176: F, t553: F, t7262: F, t303: F, t7203: F, t6923: F, t21484: F, t7909: F, t3984: F, t27340: F, t7042: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t29299 = t7899 * t7091;
    let t29300 = t6176 * t29299;
    let t29304 = t553 * t7262;
    let t29305 = t303 * t29304;
    let t29307 = t553 * t7203;
    let t29308 = t303 * t29307;
    let t29310 = t553 * t6923;
    let t29311 = t303 * t29310;
    let t29313 = t7909 * t21484;
    let t29314 = t3984 * t29313;
    let t29323 = t27340 * t7042;
    (t29299, t29300, t29304, t29305, t29307, t29308, t29310, t29311, t29313, t29314, t29323)
}
