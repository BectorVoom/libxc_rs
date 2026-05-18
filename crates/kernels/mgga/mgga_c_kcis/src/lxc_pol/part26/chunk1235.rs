//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1235/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1235<F: Float>(t27555: F, t27566: F, t3754: F, t4314: F, t27594: F, t54162: F, t7978: F, t7985: F, t7970: F, t7968: F, t12825: F, t7980: F) -> (F, F, F, F, F, F, F) {
    let t94931 = t27555 * t27566;
    let t94960 = t4314 * t3754;
    let t94966 = t27594 * t27566;
    let t94974 = t7978 * t54162 * t7985;
    let t94976 = t54162 * t7970;
    let t94977 = t7978 * t94976;
    let t94979 = t7968 * t94976;
    let t95001 = t7978 * t12825 * t7980;
    (t94931, t94960, t94966, t94974, t94977, t94979, t95001)
}
