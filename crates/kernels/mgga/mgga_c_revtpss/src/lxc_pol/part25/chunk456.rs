//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 456/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk456<F: Float>(t251: F, t785: F, t780: F, t2439: F, t212: F, t860: F, t689: F, t779: F, t887: F, t211: F, t784: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2440 = t785 * t251;
    let t2441 = t2440 * t780;
    let t2443 = 0.65049603595885220126e-3 * t2439 * t2441;
    let t2444 = t212 * t860;
    let t2445 = t2444 * t780;
    let t2446 = t689 * t2445;
    let t2448 = t779 * t887;
    let t2449 = t689 * t2448;
    let t2452 = 1.0 / t784 / t211;
    (t2440, t2441, t2443, t2444, t2445, t2446, t2448, t2449, t2452)
}
