//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 981/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk981<F: Float>(t11760: F, t3320: F, t783: F, t910: F, t2207: F, t3319: F, t10856: F, t2605: F, t938: F, t2201: F, t2842: F, t3281: F) -> (F, F, F, F, F, F, F) {
    let t11762 = t783 * t11760 * t3320;
    let t11764 = t3320 * t910;
    let t11766 = t2207 * t3319 * t11764;
    let t11768 = t10856 * t2605;
    let t11770 = t3320 * t938;
    let t11772 = t2201 * t3319 * t11770;
    let t11774 = t3281 * t2842;
    (t11762, t11764, t11766, t11768, t11770, t11772, t11774)
}
