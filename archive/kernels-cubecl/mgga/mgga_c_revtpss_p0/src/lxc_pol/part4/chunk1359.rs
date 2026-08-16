//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1359/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1359<F: Float>(t16783: F, t16786: F, t16788: F, t16790: F, t16809: F, t16814: F, t16834: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t17094: F, t17160: F, t17162: F, t17166: F, t17168: F) -> F {
    let t17169 = -t16783 - t16786 - t16788 - t16790 - t16809 - t16814 + t16834 + t16837 + t16839 + t16842 + t16844 + t16846 + t16945 + t17160 + t17162 - t17094 - t17166 - t17168;
    t17169
}
