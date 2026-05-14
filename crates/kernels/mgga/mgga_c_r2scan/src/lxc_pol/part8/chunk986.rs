//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 986/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk986<F: Float>(t8768: F, t8818: F, t8858: F, t9088: F, t9134: F, t9187: F, t9239: F, t9276: F, t9300: F, t9331: F, t9370: F, t9403: F, t9433: F, t9467: F, t9500: F, t9556: F) -> (F,) {
    let t9560 = t8768 + t8818 + t8858 + t9088 + t9134 + t9187 + t9239 + t9276 + t9300 + t9331 + t9370 + t9403 + t9433 + t9467 + t9500 + t9556;
    (t9560,)
}
