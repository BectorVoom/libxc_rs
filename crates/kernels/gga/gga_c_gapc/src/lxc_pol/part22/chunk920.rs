//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 920/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk920<F: Float>(t12040: F, t687: F, t1049: F, t10526: F, t10529: F, t2967: F, t3179: F, t3480: F, t1112: F, t8598: F, t3483: F, t8601: F, t2964: F, t3537: F, t3808: F, t4908: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12041 = t12040 * t687;
    let t12042 = t10526 * t1049;
    let t12043 = t10529 * t2967;
    let t12044 = 2.0 * t12043;
    let t12045 = t3480 * t3179;
    let t12046 = t8598 * t1112;
    let t12047 = t8601 * t3483;
    let t12048 = 2.0 * t12047;
    let t12049 = t2964 * t3537;
    let t12050 = t4908 * t3808;
    (t12041, t12042, t12043, t12044, t12045, t12046, t12047, t12048, t12049, t12050)
}
