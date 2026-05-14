//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1053/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1053<F: Float>(t126138: F, t2747: F, t31767: F, t31772: F, t2769: F, t34074: F, t10770: F, t31756: F, t4433: F, t119852: F, t4364: F, t4486: F, t4469: F, t8477: F, t31844: F, t826: F) -> (F, F, F, F, F, F) {
    let t126232 = t31767 * t2747 * t31772 * t126138;
    let t126250 = t34074 * t2769;
    let t126256 = t31767 * t10770 * t31756 * t4433;
    let t126260 = t31767 * t4364 * t119852 * t4486;
    let t126273 = t8477 * t4469;
    let t126276 = t31844 * t826;
    (t126232, t126250, t126256, t126260, t126273, t126276)
}
