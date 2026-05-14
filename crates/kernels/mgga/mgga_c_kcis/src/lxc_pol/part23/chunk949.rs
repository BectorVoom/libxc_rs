//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 949/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk949<F: Float>(t2237: F, t27484: F, t27342: F, t27416: F, t27455: F, t27459: F, t27462: F, t27465: F, t27471: F, t27477: F, t27480: F, t27483: F, t7898: F, t7908: F, t7911: F, t27375: F, t27413: F, t27452: F) -> (F, F) {
    let t27486 = 0.15445601851851851852e-3 * t2237 * t27484;
    let t27487 = 0.46336805555555555556e-3 * t7908 * t27455 - 0.46336805555555555556e-3 * t27459 * t7911 + 0.33163888888888888888e-2 * t27462 + 0.24872916666666666666e-2 * t27465 + 0.69505208333333333333e-3 * t2237 * t27416 - 0.13901041666666666667e-2 * t2237 * t27342 + 0.61836467013888888889e-4 * t27471 - 0.2782641015625e-3 * t7898 * t27342 - 0.49745833333333333332e-2 * t27477 + 0.33163888888888888888e-2 * t27480 - t27483 + t27486;
    let t27489 = t27375 + t27413 + t27452 + t27487;
    (t27486, t27489)
}
