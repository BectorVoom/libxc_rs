//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1010/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1010<F: Float>(t27932: F, t47300: F, t26009: F, t9802: F, t26004: F, t3961: F, t7252: F, t9700: F, t64: F, t9990: F, t239: F, t820: F, t9997: F, t2482: F, t596: F, t7262: F) -> (F, F, F, F, F, F) {
    let t94481 = t27932 * t47300;
    let t94483 = t9802 * t26009;
    let t94485 = t26004 * t3961;
    let t94487 = t7252 * t9700;
    let t94491 = t9990 * t64;
    let t94493 = t820 * t94491 * t239;
    let t94494 = t94493 * t9997;
    let t94497 = t2482 * t7262 * t596;
    (t94481, t94483, t94485, t94487, t94494, t94497)
}
