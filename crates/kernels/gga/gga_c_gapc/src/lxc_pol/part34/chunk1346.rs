//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1346/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1346<F: Float>(t1096: F, t11039: F, t2469: F, t12291: F, t7063: F, t972: F, t1112: F, t24906: F, t10786: F, t2964: F, t1125: F, t24004: F) -> (F, F, F, F, F) {
    let t36266 = F::new(2.0) * t2469 * t11039 * t1096;
    let t36269 = F::new(12.0) * t7063 * t12291 * t972;
    let t36270 = t24906 * t1112;
    let t36271 = t2964 * t10786;
    let t36272 = t24004 * t1125;
    (t36266, t36269, t36270, t36271, t36272)
}
