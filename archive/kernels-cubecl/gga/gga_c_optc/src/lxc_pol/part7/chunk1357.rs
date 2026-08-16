//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1357/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1357<F: Float>(t1: F, t27010: F, t15654: F, t9044: F, t123: F, t17919: F, t1900: F, t15305: F, t2860: F, t4356: F, t24502: F, t3102: F) -> (F, F, F, F, F, F) {
    let t27011 = t27010 * t1;
    let t27012 = t15654 * t9044;
    let t27017 = t17919 * t1900 * t123;
    let t27023 = t15305 * t9044;
    let t27027 = t4356 * t2860;
    let t27031 = t3102 * t24502;
    (t27011, t27012, t27017, t27023, t27027, t27031)
}
