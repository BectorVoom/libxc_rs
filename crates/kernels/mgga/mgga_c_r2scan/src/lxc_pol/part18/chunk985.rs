//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 985/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk985<F: Float>(t39894: F, t2201: F, t2719: F, t3319: F, t3320: F, t2207: F, t2526: F, t10899: F, t11764: F, t10841: F, t2842: F, t10776: F, t10810: F, t2574: F, t10708: F, t10710: F, t24912: F) -> (F, F, F, F, F, F, F) {
    let t39895 = 0.46574606203128791246e-1 * t39894;
    let t39899 = t2201 * t3319 * t3320 * t2719;
    let t39900 = 0.46574606203128791246e-1 * t39899;
    let t39903 = t2207 * t3319 * t3320 * t2526;
    let t39906 = t2207 * t10899 * t11764;
    let t39908 = t10841 * t2842;
    let t39911 = t10776 * t10810 * t2574;
    let t39912 = 0.23115257973478049502e0 * t39911;
    let t39920 = t10708 * t10710 * t24912;
    (t39895, t39900, t39903, t39906, t39908, t39912, t39920)
}
