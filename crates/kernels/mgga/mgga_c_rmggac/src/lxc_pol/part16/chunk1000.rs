//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1000/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1000<F: Float>(t1939: F, t1986: F, t7720: F, t39141: F, t9222: F, t1528: F, t236: F, t3351: F, t551: F, t7248: F, t1587: F, t618: F) -> (F, F, F, F) {
    let t46891 = t1986 * t1939;
    let t46892 = t7720 * t46891;
    let t46894 = t9222 * t39141;
    let t46899 = t3351 * t7248 * t236 * t1528 * t551;
    let t46904 = t3351 * t7248 * t236 * t618 * t1587;
    (t46892, t46894, t46899, t46904)
}
