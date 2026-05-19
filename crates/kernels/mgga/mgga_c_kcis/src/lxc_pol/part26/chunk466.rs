//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 466/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk466<F: Float>(t160: F, t531: F, t1444: F, t740: F, t833: F, t1452: F, t743: F, t1431: F, t733: F, t1438: F, t738: F, t113: F, t3754: F) -> (F, F, F, F, F, F, F, F) {
    let t4059 = t160 * t531;
    let t4060 = F::cast_from(0.15538616723388920628e-3_f64) * t4059;
    let t4061 = t740 * t1444;
    let t4062 = t4061 * t833;
    let t4073 = t743 * t1452;
    let t4081 = t733 * t1431;
    let t4089 = t738 * t1438;
    let t4093 = t113 * t3754;
    (t4059, t4060, t4061, t4062, t4073, t4081, t4089, t4093)
}
