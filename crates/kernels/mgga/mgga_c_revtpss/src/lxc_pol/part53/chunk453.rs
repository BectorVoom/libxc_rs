//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 453/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk453<F: Float>(t2789: F, t869: F, t689: F, t136: F, t251: F, t2457: F, t2710: F, t2783: F, t786: F, t231: F, t268: F, t675: F, t836: F, t72: F, t860: F, t686: F, t874: F) -> (F, F, F, F, F, F) {
    let t2790 = t869 * t2789;
    let t2791 = t689 * t2790;
    let t2793 = t251 * t136;
    let t2796 = 0.11565819519348392139e-2 * t2710 * t2793 * t2457;
    let t2797 = t2783 * t251;
    let t2798 = t786 * t2797;
    let t2801 = t268 * t675 * t836 * t231;
    let t2802 = t2798 * t2801;
    let t2804 = t860 * t72;
    let t2806 = t874 * t2804 * t686;
    (t2791, t2796, t2798, t2801, t2802, t2806)
}
