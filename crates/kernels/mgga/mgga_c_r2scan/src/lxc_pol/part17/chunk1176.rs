//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1176/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1176<F: Float>(t10710: F, t30119: F, t37586: F, t3602: F, t37755: F, t7605: F, t30691: F, t37582: F, t10708: F, t27977: F, t10810: F, t1592: F, t9380: F) -> (F, F, F, F, F) {
    let t43356 = t37586 * t10710 * t30119;
    let t43359 = t37755 * t3602 * t7605;
    let t43362 = t37582 * t10710 * t30691;
    let t43365 = t10708 * t10710 * t27977;
    let t43368 = t1592 * t10810 * t9380;
    (t43356, t43359, t43362, t43365, t43368)
}
