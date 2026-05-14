//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1414/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1414<F: Float>(t112566: F, t6967: F, t1091: F, t111668: F, t112384: F, t112602: F, t112630: F, t112641: F, t112643: F, t112647: F, t1212: F, t126162: F, t126282: F, t1466: F, t193: F, t2665: F, t28985: F, t29000: F, t31952: F, t3746: F, t4309: F, t6216: F, t6222: F, t830: F) -> (F,) {
    let t128651 = t112566 * t6967;
    let t128660 = -t6216 * t2665 * t112384 * t1091 / 9.0 - 2.0 * t126162 - t830 * t31952 - t112602 - t6216 * t2665 * t111668 * t1091 / 9.0 + 2.0 / 9.0 * t29000 * t2665 * t28985 * t3746 + t128651 / 27.0 + 4.0 / 27.0 * t112630 - 2.0 / 3.0 * t1466 * t193 * t6222 * t4309 * t1212 - 2.0 * t126282 + t112641 + t112643 - t112647;
    (t128660,)
}
