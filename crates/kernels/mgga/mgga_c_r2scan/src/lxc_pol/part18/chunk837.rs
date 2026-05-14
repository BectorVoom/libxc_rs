//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 837/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk837<F: Float>(t9793: F, t9794: F, t9798: F, t9799: F, t9802: F, t9810: F, t9818: F, t9829: F, t1569: F, t3052: F, t2987: F, t352: F, t104: F, t114: F, t97: F) -> (F, F, F, F, F) {
    let t9832 = t9793 + t9794 + t9798 + t9799 + t9802 + t9810 + t9818 + t9829;
    let t10024 = t1569 * t3052;
    let t10533 = t352 * t2987;
    let t10609 = t104 * t114;
    let t10610 = t97 * t10609;
    (t9832, t10024, t10533, t10609, t10610)
}
