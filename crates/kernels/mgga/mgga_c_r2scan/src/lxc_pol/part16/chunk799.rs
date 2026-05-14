//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 799/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk799<F: Float>(t3179: F, t780: F, t113: F, t8735: F, t6086: F, t6085: F, t8740: F) -> (F, F, F, F) {
    let t9240 = t3179 * t780;
    let t9242 = t8735 * t113;
    let t9243 = t6086 * t9242;
    let t9244 = t6085 * t9243;
    let t9246 = t8740 * t113;
    (t9240, t9242, t9244, t9246)
}
