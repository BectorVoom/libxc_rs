//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1444/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1444<F: Float>(t22252: F, t543: F, t1390: F, t828: F, t221: F, t4019: F, t6844: F, t4018: F, t14045: F, t6869: F, t3992: F, t2661: F) -> (F, F, F, F) {
    let t22253 = t22252 * t543;
    let t22255 = t1390 * t828 * t22253;
    let t22259 = t4019 * t221 * t6844;
    let t22260 = t4018 * t22259;
    let t22262 = t14045 * t6869;
    let t22263 = t3992 * t22262;
    let t22264 = t2661 * t22263;
    (t22253, t22255, t22260, t22264)
}
