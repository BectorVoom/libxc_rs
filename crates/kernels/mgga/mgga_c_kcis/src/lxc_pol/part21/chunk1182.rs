//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1182/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1182<F: Float>(t10497: F, t1778: F, t3329: F, t5034: F, t1239: F, t15469: F, t1262: F, t5336: F, t110: F, t287: F) -> (F, F, F, F, F) {
    let t46026 = t1778 * t10497;
    let t46041 = t5034 * t3329;
    let t46577 = t15469 * t1239;
    let t46849 = t5336 * t1262;
    let t46978 = t110 * t287;
    (t46026, t46041, t46577, t46849, t46978)
}
