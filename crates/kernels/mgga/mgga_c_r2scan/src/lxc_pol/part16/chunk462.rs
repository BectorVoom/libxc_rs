//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 462/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk462<F: Float>(t322: F, t1013: F, t833: F, t829: F, t1300: F, t2394: F, t327: F, t834: F, t330: F, t1018: F, t837: F, t2393: F) -> (F, F, F, F, F, F) {
    let t332 = 0.25e1 < t322;
    let t2397 = t1013 * t833;
    let t2400 = t1013 * t829;
    let t2405 = -0.64e0 * t2394 * t327 - 0.128e1 * t2397 * t829 - 0.128e1 * t1300 * t2400 - 0.64e0 * t834 * t2394;
    let t2406 = t2405 * t330;
    let t2407 = t1018 * t837;
    let t2408 = t2407 * t330;
    let t2410 = piecewise3(t332, 0.0, t2393);
    (t2397, t2400, t2405, t2406, t2408, t2410)
}
