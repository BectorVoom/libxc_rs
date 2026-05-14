//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 964/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk964<F: Float>(t1147: F, t803: F, t1058: F, t568: F, t2507: F, t42: F, t1259: F, t955: F, t14: F, t4494: F, t26: F, t4635: F, t30: F, t4827: F, t1447: F, t41: F) -> (F, F, F, F, F, F, F, F) {
    let t12389 = t1147 * t803;
    let t12419 = t1058 * t568;
    let t12431 = t2507 * t42;
    let t12919 = t1259 * t955;
    let t13925 = t14 * t4494;
    let t14431 = t26 * t4635;
    let t16036 = t30 * t4827;
    let t16046 = 1.0 / t1447 / t41;
    (t12389, t12419, t12431, t12919, t13925, t14431, t16036, t16046)
}
