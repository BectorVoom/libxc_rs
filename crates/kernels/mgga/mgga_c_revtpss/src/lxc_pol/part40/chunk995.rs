//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 995/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk995<F: Float>(t788: F, t9288: F, t787: F, t2453: F, t861: F, t2458: F, t2761: F, t786: F, t789: F, t212: F, t2760: F, t780: F, t689: F, t785: F, t860: F, t2439: F) -> (F, F, F, F, F) {
    let t11015 = t788 * t9288;
    let t11017 = 0.30356481678079769392e-1 * t787 * t11015;
    let t11018 = t2453 * t861;
    let t11019 = t11018 * t2458;
    let t11021 = t786 * t2761;
    let t11022 = t11021 * t789;
    let t11024 = t212 * t2760;
    let t11025 = t11024 * t780;
    let t11026 = t689 * t11025;
    let t11028 = t785 * t860;
    let t11029 = t11028 * t780;
    let t11030 = t2439 * t11029;
    (t11017, t11019, t11022, t11026, t11030)
}
