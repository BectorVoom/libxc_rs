//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 924/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk924<F: Float>(t12050: F, t3808: F, t687: F, t4915: F, t1049: F, t3537: F, t1616: F, t1112: F, t3179: F, t1611: F, t3822: F, t11314: F, t11318: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12051 = 2.0 * t12050;
    let t12052 = t3808 * t687;
    let t12053 = t4915 * t12052;
    let t12054 = 6.0 * t12053;
    let t12055 = t3537 * t1049;
    let t12056 = t1616 * t12055;
    let t12057 = 2.0 * t12056;
    let t12058 = t1112 * t3179;
    let t12059 = t1616 * t12058;
    let t12060 = 2.0 * t12059;
    let t12061 = t1611 * t3822;
    let t12062 = t3822 * t687;
    let t12063 = t1616 * t12062;
    let t12064 = 2.0 * t12063;
    let t12068 = 0.34752370105806885418e-3 * t11314;
    let t12069 = 0.34752370105806885418e-3 * t11318;
    (t12051, t12052, t12053, t12054, t12055, t12056, t12057, t12058, t12059, t12060, t12061, t12062, t12063, t12064, t12068, t12069)
}
