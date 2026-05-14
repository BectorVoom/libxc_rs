//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 541/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk541<F: Float>(t1052: F, t369: F, t361: F, t351: F, t1065: F, t126: F, t906: F, t247: F) -> (F, F, F, F, F) {
    let t3104 = t1052 * t369;
    let t3105 = t361 * t3104;
    let t3106 = t351 * t3105;
    let t3109 = t126 * t1065;
    let t3110 = t3109 * t906;
    let t3111 = t247 * t3110;
    (t3104, t3105, t3106, t3109, t3111)
}
