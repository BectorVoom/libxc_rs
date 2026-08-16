//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 866/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk866<F: Float>(t2619: F, t9083: F, t7939: F, t8769: F, t7921: F, t1084: F, t9253: F, t2579: F, t966: F, t1038: F, t8133: F) -> (F, F, F, F, F, F) {
    let t9856 = t2619 * t9083;
    let t9857 = t9856 * t7939;
    let t9859 = t2619 * t8769;
    let t9860 = t9859 * t7921;
    let t9862 = t1084 * t9253;
    let t9863 = t2579 * t966;
    let t9864 = t1038 * t8133;
    let t9865 = t9863 * t9864;
    (t9857, t9860, t9862, t9863, t9864, t9865)
}
