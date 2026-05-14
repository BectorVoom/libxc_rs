//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1158/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1158<F: Float>(t97156: F, t97202: F, t97249: F, t97297: F, t3567: F, t8945: F, t26894: F, t29199: F, t3596: F, t37885: F, t2149: F, t1294: F, t5464: F, t1210: F, t1203: F, t21471: F) -> (F, F, F, F, F, F, F) {
    let t97299 = t97156 + t97202 + t97249 + t97297;
    let t97304 = t3567 * t8945;
    let t97308 = t26894 * t29199;
    let t97312 = t37885 * t3596;
    let t97313 = t2149 * t97312;
    let t97314 = t5464 * t1294;
    let t97318 = t1210 * t29199;
    let t97319 = t21471 * t1203;
    (t97299, t97304, t97308, t97313, t97314, t97318, t97319)
}
