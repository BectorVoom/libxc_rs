//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 946/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk946<F: Float>(t25273: F, t533: F, t816: F, t540: F, t7021: F, t1372: F, t3961: F, t7252: F, t1389: F, t7269: F, t2736: F, t2689: F, t7256: F, t2018: F, t3951: F, t807: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26002 = t25273 * t533 * t816;
    let t26004 = t7021 * t540;
    let t26005 = t26004 * t1372;
    let t26007 = t7252 * t3961;
    let t26009 = t7269 * t1389;
    let t26010 = t2736 * t26009;
    let t26012 = t2689 * t7256;
    let t26014 = t2018 * t3951;
    let t26015 = t807 * t26014;
    (t26002, t26004, t26005, t26007, t26009, t26010, t26012, t26014, t26015)
}
