//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 836/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk836<F: Float>(t9824: F, t2266: F, t2526: F, t2854: F, t4873: F, t5039: F, t7156: F, t8653: F, t8654: F, t8655: F, t8656: F, t8657: F, t8658: F, t881: F, t9069: F, t9072: F) -> (F,) {
    let t9825 = 3.0 * t9824;
    let t9827 = t2266 * t2854 * t2526;
    let t9828 = 6.0 * t9827;
    let t9829 = -0.4726e1 * t881 * t9069 - 0.4726e1 * t881 * t9072 + t8653 + t8654 + t8655 - t4873 - t9825 + t7156 + t8656 + t8657 - t8658 - t9828 - t5039;
    (t9829,)
}
