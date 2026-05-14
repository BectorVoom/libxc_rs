//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1054/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1054<F: Float>(t291: F, t9916: F, t417: F, t9874: F, t209: F, t736: F, t1003: F, t167: F, t1646: F, t3040: F, t2887: F, t1141: F, t14663: F, t10497: F, t1778: F, t3329: F, t5034: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44575 = t9916 * t291;
    let t44657 = t417 * t9874;
    let t44682 = t209 * t736;
    let t44684 = t167 * t1003;
    let t44743 = t1646 * t3040;
    let t44756 = t2887 * t291;
    let t46015 = t14663 * t1141;
    let t46026 = t1778 * t10497;
    let t46041 = t5034 * t3329;
    (t44575, t44657, t44682, t44684, t44743, t44756, t46015, t46026, t46041)
}
