//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1365/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1365<F: Float>(t342: F, t43471: F, t3154: F, t43351: F, t16551: F, t994: F, t16558: F, t11627: F, t42859: F, t11631: F, t3494: F, t3519: F) -> (F, F, F, F, F, F, F) {
    let t43472 = t342 * t43471;
    let t43473 = t43351 * t3154;
    let t43520 = t994 * t16551;
    let t43524 = t994 * t16558;
    let t43536 = t42859 * t11627;
    let t43537 = t342 * t43536;
    let t43538 = t43351 * t11631;
    let t43752 = F::cast_from(1.0_f64) / t3519 / t3494;
    (t43472, t43473, t43520, t43524, t43537, t43538, t43752)
}
