//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1033/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1033<F: Float>(t19790: F, t921: F, t2654: F, t6212: F, t2625: F, t2634: F, t2612: F, t2531: F, t2599: F, t3433: F, t10855: F, t110: F) -> (F, F, F, F, F, F, F, F) {
    let t25397 = t19790 * t921;
    let t25480 = t6212 * t2654;
    let t25486 = t6212 * t2625;
    let t25499 = t6212 * t2634;
    let t25503 = t6212 * t2612;
    let t25737 = t6212 * t2531;
    let t25826 = t3433 * t2599;
    let t25851 = t10855 * t110;
    (t25397, t25480, t25486, t25499, t25503, t25737, t25826, t25851)
}
