//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 435/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk435<F: Float>(t136: F, t251: F, t2457: F, t2710: F, t2783: F, t786: F, t2470: F, t874: F, t875: F, t2718: F, t1941: F, t268: F, t271: F, t1065: F, t159: F, t631: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2793 = t251 * t136;
    let t2796 = 0.11565819519348392139e-2 * t2710 * t2793 * t2457;
    let t2797 = t2783 * t251;
    let t2798 = t786 * t2797;
    let t2810 = 0.13009920719177044025e-1 * t874 * t875 * t2470;
    let t2811 = t2718 * t251;
    let t2846 = t268 * t1941 * t271;
    let t2847 = 0.23744444444444444444e-1 * t2846;
    let t2850 = t159 * t1065;
    let t2851 = t631 * t631;
    (t2793, t2796, t2797, t2798, t2810, t2811, t2846, t2847, t2850, t2851)
}
