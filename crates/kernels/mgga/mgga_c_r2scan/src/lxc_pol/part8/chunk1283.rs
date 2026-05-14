//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1283/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1283<F: Float>(t22731: F, t9381: F, t3190: F, t6212: F, t20589: F, t6211: F, t2169: F, t9477: F, t29731: F, t538: F, t6155: F, t2155: F, t29497: F, t29501: F, t7601: F, t8232: F) -> (F, F, F, F, F, F, F) {
    let t30113 = t22731 * t9381;
    let t30119 = t6212 * t3190;
    let t30121 = t20589 * t6211 * t30119;
    let t30123 = t2169 * t9477;
    let t30132 = t6155 * t538 * t29731;
    let t30134 = t2155 * t29497;
    let t30136 = t2155 * t29501;
    let t30138 = t7601 * t8232;
    (t30113, t30121, t30123, t30132, t30134, t30136, t30138)
}
