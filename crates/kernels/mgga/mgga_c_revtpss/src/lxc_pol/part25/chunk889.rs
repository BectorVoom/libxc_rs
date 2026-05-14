//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 889/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk889<F: Float>(t11244: F, t3144: F, t11240: F, t1043: F, t3151: F, t373: F, t3153: F, t73: F) -> (F, F, F, F) {
    let t11245 = t3144 * t11244;
    let t11246 = t11240 * t11245;
    let t11247 = t3151 * t1043;
    let t11248 = t373 * t11247;
    let t11249 = t3153 * t73;
    (t11246, t11247, t11248, t11249)
}
