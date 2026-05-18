//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1008/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1008<F: Float>(t2580: F, t7943: F, t147: F, t786: F, t3412: F, t8133: F, t4978: F, t7073: F, t2188: F, t314: F, t959: F, t7591: F) -> (F, F, F, F, F, F) {
    let t16798 = t2580 * t7943;
    let t16826 = t147 * t786;
    let t17713 = t3412 * t8133;
    let t17760 = t7073 * t4978;
    let t17819 = t2188 * t959 * t314;
    let t17874 = t7591 * t314;
    (t16798, t16826, t17713, t17760, t17819, t17874)
}
