//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 631/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk631<F: Float>(t3362: F, t3698: F, t1251: F, t3172: F, t1247: F, t1032: F, t1204: F, t1246: F, t1234: F, t1260: F) -> (F, F, F, F, F, F) {
    let t3699 = t3698 * t3362;
    let t3704 = t3172 * t1251;
    let t3705 = t1247 * t3704;
    let t3707 = t1204 * t1032;
    let t3708 = t3707 * t1246;
    let t3711 = t1234 * t1260;
    (t3699, t3704, t3705, t3707, t3708, t3711)
}
