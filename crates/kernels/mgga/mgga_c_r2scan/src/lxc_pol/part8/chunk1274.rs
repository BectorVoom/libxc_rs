//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1274/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1274<F: Float>(t19905: F, t2155: F, t29467: F, t3055: F, t538: F, t6191: F, t6194: F, t2530: F, t7338: F, t6086: F, t6535: F, t3016: F, t494: F, t113: F, t6063: F, t7601: F, t8067: F) -> (F, F, F, F, F, F, F) {
    let t29713 = t2155 * t19905 * t29467;
    let t29720 = t6191 * t538 * t3055 * t6194;
    let t29726 = t7338 * t2530;
    let t29728 = t6535 * t6086 * t29726;
    let t29730 = t3016 * t494;
    let t29731 = t29730 * t113;
    let t29733 = t2155 * t6063 * t29731;
    let t29742 = t7601 * t8067;
    (t29713, t29720, t29726, t29728, t29731, t29733, t29742)
}
