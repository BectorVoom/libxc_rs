//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 782/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk782<F: Float>(t16004: F, t3192: F, t1128: F, t5407: F, t3186: F, t1148: F, t5275: F, t911: F, t1168: F, t5279: F, t871: F, t4492: F, t4509: F, t4501: F, t4512: F, t1157: F, t5421: F) -> (F, F, F, F, F, F, F, F) {
    let t16005 = t3192 * t16004;
    let t16007 = t1128 * t5407;
    let t16008 = t3186 * t16007;
    let t16011 = t1148 * t5275 * t911;
    let t16024 = t1168 * t5279 * t871;
    let t16035 = t4492 * t4509;
    let t16037 = t4501 * t4512;
    let t16055 = t5421 * t1157;
    (t16005, t16007, t16008, t16011, t16024, t16035, t16037, t16055)
}
