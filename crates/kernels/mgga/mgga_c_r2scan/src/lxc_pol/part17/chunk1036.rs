//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1036/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1036<F: Float>(t3602: F, t37755: F, t7605: F, t10710: F, t30691: F, t37582: F, t10708: F, t27977: F, t10810: F, t1592: F, t9380: F, t3190: F, t3319: F, t3320: F, t5103: F, t22790: F, t30057: F, t3332: F) -> (F, F, F, F, F, F) {
    let t43359 = t37755 * t3602 * t7605;
    let t43362 = t37582 * t10710 * t30691;
    let t43365 = t10708 * t10710 * t27977;
    let t43368 = t1592 * t10810 * t9380;
    let t43372 = t5103 * t3319 * t3320 * t3190;
    let t43376 = t22790 * t3332 * t30057;
    (t43359, t43362, t43365, t43368, t43372, t43376)
}
