//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 799/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk799<F: Float>(t2042: F, t2049: F, t240: F, t2815: F, t5527: F, t5532: F, t802: F, t9695: F, t9697: F, t9698: F, t9701: F, t9719: F, t9758: F, t9760: F, t9763: F, t9772: F) -> (F,) {
    let t9776 = t9695 - t9697 - t9698 + t9701 - t9719 + t240 * (-t2042 * t9772 - t2049 * t9760 - t2815 * t5527 + 2.0 * t5532 * t9763 + t802 * t9758 - t9695 + t9697 + t9698 - t9701 + t9719);
    (t9776,)
}
