//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 555/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk555<F: Float>(t1004: F, t996: F, t390: F, t1020: F, t997: F, t3055: F, t383: F, t1039: F, t1029: F, t1032: F, t993: F, t1205: F) -> (F, F, F, F, F, F) {
    let t3770 = t1004 * t996;
    let t3772 = F::new(0.60023625365297631762e-2) * t3770 * t390;
    let t3773 = t997 * t1020;
    let t3775 = t3055 * t383;
    let t3777 = F::new(0.12862205435420921092e-2) * t3775 * t1039;
    let t3778 = t997 * t1029;
    let t3782 = F::new(0.30011812682648815881e-2) * t1032 * t993;
    let t3783 = t997 * t1205;
    (t3772, t3773, t3777, t3778, t3782, t3783)
}
