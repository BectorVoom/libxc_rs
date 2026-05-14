//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 819/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk819<F: Float>(t11294: F, t1616: F, t1611: F, t3721: F, t687: F, t1736: F, t1971: F) -> (F, F, F, F, F, F, F) {
    let t11295 = t1616 * t11294;
    let t11296 = 4.0 * t11295;
    let t11297 = t1611 * t3721;
    let t11298 = t3721 * t687;
    let t11299 = t1616 * t11298;
    let t11300 = 2.0 * t11299;
    let t11301 = t1736 * M_PI;
    let t11302 = t1971 * t11301;
    (t11295, t11296, t11297, t11298, t11299, t11300, t11302)
}
