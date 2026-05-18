//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 917/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk917<F: Float>(t1005: F, t3756: F, t3652: F, t3775: F, t3657: F, t1086: F, t3670: F, t1113: F, t3700: F, t3740: F, t957: F, t1163: F, t1165: F, t3439: F, t4162: F) -> (F, F, F, F, F, F, F) {
    let t14003 = t1005 * t3756;
    let t14005 = t3775 * t3652;
    let t14015 = t3775 * t3657;
    let t14017 = t3670 * t1086;
    let t14019 = t3700 * t1113;
    let t14022 = t3740 * t957;
    let t14044 = t1163 * t1165 * t3439 * t4162;
    (t14003, t14005, t14015, t14017, t14019, t14022, t14044)
}
