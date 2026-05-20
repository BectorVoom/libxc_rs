//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1539/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1539<F: Float>(t24042: F, t994: F, t23959: F, t378: F, t4746: F, t6343: F, t79862: F, t1647: F, t1678: F, t6235: F, t342: F, t25026: F, t3801: F) -> (F, F, F, F, F, F, F, F) {
    let t80810 = t994 * t24042;
    let t80833 = t23959 * t378;
    let t80901 = t4746 * t6343;
    let t80921 = t79862 * t378;
    let t80983 = t1647 * t6343;
    let t80992 = t6235 * t1678;
    let t81052 = t342 * t24042;
    let t81139 = t25026 * t3801;
    (t80810, t80833, t80901, t80921, t80983, t80992, t81052, t81139)
}
