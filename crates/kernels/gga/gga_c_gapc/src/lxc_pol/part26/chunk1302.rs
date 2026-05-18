//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1302/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1302<F: Float>(t11291: F, t23726: F, t1616: F, t2011: F, t3721: F, t3659: F, t4915: F, t3449: F, t15430: F, t11298: F, t4908: F, t11294: F) -> (F, F, F, F, F, F, F) {
    let t36055 = F::new(12.0) * t23726 * t11291;
    let t36058 = F::new(2.0) * t1616 * t3721 * t2011;
    let t36067 = F::new(6.0) * t4915 * t3659 * t2011;
    let t36068 = t3449 * t3449;
    let t36072 = F::new(2.0) * t15430 * t3659;
    let t36074 = F::new(4.0) * t4908 * t11298;
    let t36078 = F::new(8.0) * t4908 * t11294;
    (t36055, t36058, t36067, t36068, t36072, t36074, t36078)
}
