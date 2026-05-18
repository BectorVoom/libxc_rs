//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1181/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1181<F: Float>(t15104: F, t17623: F, t18198: F, t5236: F, t5238: F, t8: F, t15063: F, t17622: F, t17627: F, t43671: F, t11782: F, t18213: F) -> (F, F, F, F, F) {
    let t53776 = t17623 * t15104;
    let t53793 = t5236 * t5238 * t18198 * t8;
    let t53812 = t17622 * t15063;
    let t53823 = t43671 * t17627;
    let t53825 = t11782 * t18213;
    (t53776, t53793, t53812, t53823, t53825)
}
