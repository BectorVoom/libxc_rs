//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 938/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk938<F: Float>(t481: F, t9235: F, t2526: F, t2841: F, t3216: F, t494: F, t113: F, t2530: F, t7338: F, t3016: F, t9272: F, t28325: F, t2567: F, t3056: F, t3071: F, t27914: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t29496 = t9235 * t481;
    let t29500 = t2841 * t2526;
    let t29699 = t3216 * t494;
    let t29700 = t29699 * t113;
    let t29726 = t7338 * t2530;
    let t29730 = t3016 * t494;
    let t29731 = t29730 * t113;
    let t29764 = t9272 * t481;
    let t29775 = t28325 * t113;
    let t29779 = t2567 * t2526;
    let t29783 = t3056 * t481;
    let t29837 = t3071 * t481;
    let t29936 = t27914 * t113;
    (t29496, t29500, t29700, t29726, t29731, t29764, t29775, t29779, t29783, t29837, t29936)
}
