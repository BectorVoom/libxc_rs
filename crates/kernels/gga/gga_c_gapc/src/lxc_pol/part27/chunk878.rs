//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 878/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk878<F: Float>(t3750: F, t871: F, t3388: F, t3392: F, t3751: F, t3769: F, t949: F, t1084: F, t11430: F, t10079: F, t11597: F, t3402: F, t3408: F, t1936: F, t7073: F, t1453: F, t291: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11862 = t871 * t3750;
    let t11863 = t11862 * t3388;
    let t11865 = t3751 * t3392;
    let t11867 = t3769 * t949;
    let t11869 = t1084 * t11430;
    let t11870 = t11869 * t10079;
    let t11872 = t3402 * t11597;
    let t11873 = t11872 * t3408;
    let t11875 = t7073 * t1936;
    let t11876 = t1453 * t291;
    (t11862, t11863, t11865, t11867, t11869, t11870, t11872, t11873, t11875, t11876)
}
