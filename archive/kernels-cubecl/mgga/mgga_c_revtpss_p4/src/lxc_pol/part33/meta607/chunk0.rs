//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2032/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2032<F: Float>(t2149: F, t97312: F, t1294: F, t5464: F, t1210: F, t29199: F, t1203: F, t21471: F, t3596: F, t7627: F, t26936: F, t3566: F) -> (F, F, F, F, F, F) {
    let t97313 = t2149 * t97312;
    let t97314 = t5464 * t1294;
    let t97318 = t1210 * t29199;
    let t97319 = t21471 * t1203;
    let t97332 = t3596 * t7627;
    let t97343 = t3566 * t26936;
    (t97313, t97314, t97318, t97319, t97332, t97343)
}
