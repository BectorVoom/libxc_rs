//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1034/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1034<F: Float>(t11643: F, t22731: F, t11654: F, t6395: F, t10869: F, t7601: F, t10811: F, t2651: F, t10903: F, t11764: F, t2207: F, t2147: F, t26307: F, t3332: F, t261: F, t3299: F, t7390: F) -> (F, F, F, F, F, F, F) {
    let t40151 = t22731 * t11643;
    let t40153 = t6395 * t11654;
    let t40155 = t7601 * t10869;
    let t40157 = t2651 * t10811;
    let t40162 = t2207 * t10903 * t11764;
    let t40165 = t2147 * t3332 * t26307;
    let t40175 = t3299 * t261 * t7390;
    (t40151, t40153, t40155, t40157, t40162, t40165, t40175)
}
