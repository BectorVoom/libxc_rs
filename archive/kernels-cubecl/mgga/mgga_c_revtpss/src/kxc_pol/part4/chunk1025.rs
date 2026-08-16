//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1025/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1025<F: Float>(t2461: F, t2471: F, t788: F, t9288: F, t787: F, t2453: F, t861: F, t2458: F, t2761: F, t786: F, t789: F, t212: F, t2760: F) -> (F, F, F, F, F) {
    let t11013 = t2461 * t2471;
    let t11015 = t788 * t9288;
    let t11017 = F::cast_from(0.30356481678079769392e-1_f64) * t787 * t11015;
    let t11018 = t2453 * t861;
    let t11019 = t11018 * t2458;
    let t11021 = t786 * t2761;
    let t11022 = t11021 * t789;
    let t11024 = t212 * t2760;
    (t11013, t11017, t11019, t11022, t11024)
}
