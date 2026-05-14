//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 767/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk767<F: Float>(t113: F, t8691: F, t3052: F, t494: F, t2530: F, t285: F, t3055: F, t3270: F, t983: F, t1561: F, t3060: F, t3229: F, t498: F, t920: F, t938: F) -> (F, F, F, F, F, F, F, F) {
    let t8692 = t8691 * t113;
    let t8694 = t3052 * t494;
    let t8698 = t285 * t2530;
    let t8701 = t3055 * t494;
    let t8707 = t3270 * t983;
    let t8714 = t1561 * t3060;
    let t8723 = t498 * t3229;
    let t8735 = t938 * t920;
    (t8692, t8694, t8698, t8701, t8707, t8714, t8723, t8735)
}
