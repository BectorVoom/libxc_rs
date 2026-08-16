//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1194/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1194<F: Float>(t27556: F, t27563: F, t27607: F, t27566: F, t27606: F, t27555: F, t18171: F, t27568: F, t27567: F, t27594: F, t54162: F, t7978: F, t7985: F) -> (F, F, F, F, F, F, F, F) {
    let t94916 = t27556 * t27563;
    let t94919 = t27607 * t27563;
    let t94928 = t27606 * t27566;
    let t94931 = t27555 * t27566;
    let t94934 = t18171 * t27568;
    let t94935 = t27567 * t94934;
    let t94966 = t27594 * t27566;
    let t94974 = t7978 * t54162 * t7985;
    (t94916, t94919, t94928, t94931, t94934, t94935, t94966, t94974)
}
