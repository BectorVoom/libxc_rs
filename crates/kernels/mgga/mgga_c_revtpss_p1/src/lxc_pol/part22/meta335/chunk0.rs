//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1792/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1792<F: Float>(t11025: F, t689: F, t785: F, t860: F, t780: F, t2439: F, t2772: F, t779: F, t781: F, t9292: F, t861: F, t867: F) -> (F, F, F, F, F, F, F, F) {
    let t11026 = t689 * t11025;
    let t11028 = t785 * t860;
    let t11029 = t11028 * t780;
    let t11030 = t2439 * t11029;
    let t11036 = t779 * t2772;
    let t11037 = t689 * t11036;
    let t11040 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t781;
    let t11043 = t861 * t867;
    (t11026, t11028, t11029, t11030, t11036, t11037, t11040, t11043)
}
