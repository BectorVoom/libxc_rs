//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1070/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1070<F: Float>(t27567: F, t94934: F, t27566: F, t27594: F, t54162: F, t7978: F, t7985: F, t7970: F, t7968: F, t27601: F, t27607: F, t18210: F, t27616: F, t12825: F, t7980: F, t27637: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94935 = t27567 * t94934;
    let t94966 = t27594 * t27566;
    let t94974 = t7978 * t54162 * t7985;
    let t94976 = t54162 * t7970;
    let t94977 = t7978 * t94976;
    let t94979 = t7968 * t94976;
    let t94981 = t27607 * t27601;
    let t94988 = t18210 * t27616;
    let t94989 = t7978 * t94988;
    let t94991 = t7968 * t94988;
    let t95001 = t7978 * t12825 * t7980;
    let t95004 = t7978 * t18210 * t27637;
    (t94935, t94966, t94974, t94977, t94979, t94981, t94989, t94991, t95001, t95004)
}
