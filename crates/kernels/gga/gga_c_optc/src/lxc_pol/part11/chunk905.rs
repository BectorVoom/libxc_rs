//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 905/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk905<F: Float>(t3793: F, t4920: F, t845: F, t16654: F, t16657: F, t16676: F, t16828: F, t16860: F, t16864: F, t16866: F, t16869: F, t16877: F, t16949: F, t16953: F) -> (F, F, F) {
    let t17041 = t3793 * t4920;
    let t17043 = F::new(0.35089340384731224426e1) * t845 * t17041;
    let t17044 = t16828 + t16860 + t16864 + t17043 - t16949 - t16953 - t16866 - t16657 + t16676 - t16869 - t16654 + t16877;
    (t17041, t17043, t17044)
}
