//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 951/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk951<F: Float>(t11018: F, t2458: F, t2761: F, t786: F, t789: F, t212: F, t2760: F, t780: F, t689: F, t785: F, t860: F, t2439: F, t2772: F, t779: F, t781: F, t9292: F) -> (F, F, F, F, F, F) {
    let t11019 = t11018 * t2458;
    let t11021 = t786 * t2761;
    let t11022 = t11021 * t789;
    let t11024 = t212 * t2760;
    let t11025 = t11024 * t780;
    let t11026 = t689 * t11025;
    let t11028 = t785 * t860;
    let t11029 = t11028 * t780;
    let t11030 = t2439 * t11029;
    let t11036 = t779 * t2772;
    let t11037 = t689 * t11036;
    let t11040 = 0.17073386770573548589e-1 * t9292 * t781;
    (t11019, t11022, t11026, t11030, t11037, t11040)
}
