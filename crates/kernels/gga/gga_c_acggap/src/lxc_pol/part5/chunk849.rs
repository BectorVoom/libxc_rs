//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 849/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk849<F: Float>(t13957: F, t425: F, t431: F, t438: F, t1200: F, t3670: F, t1205: F, t1032: F, t3292: F, t1005: F, t3732: F, t3811: F, t3756: F, t3652: F, t3775: F, t3657: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13958 = t13957 * t425;
    let t13960 = t13957 * t431;
    let t13962 = t13957 * t438;
    let t13964 = t3670 * t1200;
    let t13966 = t3670 * t1205;
    let t13974 = t1032 * t3292;
    let t13985 = t1005 * t3732;
    let t14001 = t1005 * t3811;
    let t14003 = t1005 * t3756;
    let t14005 = t3775 * t3652;
    let t14015 = t3775 * t3657;
    (t13958, t13960, t13962, t13964, t13966, t13974, t13985, t14001, t14003, t14005, t14015)
}
