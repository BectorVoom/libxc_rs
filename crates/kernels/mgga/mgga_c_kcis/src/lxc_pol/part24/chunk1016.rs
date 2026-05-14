//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1016/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1016<F: Float>(t1003: F, t1704: F, t209: F, t736: F, t2887: F, t291: F, t10497: F, t1778: F, t3329: F, t5034: F, t110: F, t287: F) -> (F, F, F, F, F, F) {
    let t44658 = t1704 * t1003;
    let t44682 = t209 * t736;
    let t44756 = t2887 * t291;
    let t46026 = t1778 * t10497;
    let t46041 = t5034 * t3329;
    let t46978 = t110 * t287;
    (t44658, t44682, t44756, t46026, t46041, t46978)
}
