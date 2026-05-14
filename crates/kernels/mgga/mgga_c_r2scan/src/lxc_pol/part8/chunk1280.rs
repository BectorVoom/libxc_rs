//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1280/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1280<F: Float>(t2731: F, t7250: F, t2147: F, t28005: F, t6086: F, t26282: F, t7926: F, t29731: F, t6093: F, t2155: F, t29783: F, t6063: F, t113: F, t3190: F, t494: F, t19865: F) -> (F, F, F, F, F, F) {
    let t29966 = t7250 * t2731;
    let t29993 = t2147 * t6086 * t28005;
    let t29998 = t26282 * t7926;
    let t30001 = t6093 * t6086 * t29731;
    let t30004 = t2155 * t6063 * t29783;
    let t30007 = t3190 * t494 * t113;
    let t30009 = t19865 * t6086 * t30007;
    (t29966, t29993, t29998, t30001, t30004, t30009)
}
