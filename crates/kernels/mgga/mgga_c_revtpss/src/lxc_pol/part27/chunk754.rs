//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 754/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk754<F: Float>(t1359: F, t9292: F, t1363: F, t9288: F, t1362: F, t3911: F, t3920: F, t3957: F, t3961: F, t124: F, t9628: F, t800: F, t3829: F, t4011: F, t547: F, t807: F) -> (F, F, F, F, F, F, F, F) {
    let t9691 = 0.17073386770573548589e-1 * t9292 * t1359;
    let t9692 = t1363 * t9288;
    let t9694 = 0.30356481678079769392e-1 * t1362 * t9692;
    let t9695 = t3911 * t3920;
    let t9697 = t3957 * t3961;
    let t9699 = t124 * t9628;
    let t9700 = t800 * t9699;
    let t9703 = t4011 * t3829;
    let t9704 = t547 * t9703;
    let t9705 = t807 * t9704;
    (t9691, t9692, t9694, t9695, t9697, t9700, t9703, t9705)
}
