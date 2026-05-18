//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1124/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1124<F: Float>(t1908: F, t505: F, t647: F, t8715: F, t2999: F, t5216: F, t1648: F, t3005: F, t154: F, t3949: F, t126: F, t632: F) -> (F, F, F, F, F) {
    let t27754 = t505 * t1908 * t647 * t8715;
    let t27867 = t2999 * t5216;
    let t27868 = t1648 * t3005 * t27867;
    let t27889 = t154 * t3949;
    let t27935 = t632 * t126;
    (t27754, t27867, t27868, t27889, t27935)
}
