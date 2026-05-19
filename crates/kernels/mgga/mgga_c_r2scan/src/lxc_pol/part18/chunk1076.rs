//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1076/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1076<F: Float>(t38189: F, t10949: F, t10992: F, t2315: F, t3446: F, t23194: F, t263: F, t3438: F, t6874: F, t10968: F, t6262: F, t6855: F) -> (F, F, F, F) {
    let t38190 = F::cast_from(0.23080304851772712107e1_f64) * t38189;
    let t38211 = t3446 * t10992 * t10949 * t2315;
    let t38225 = t3446 * t263 * t23194 * t3438 * t6874;
    let t38226 = F::cast_from(0.91462949374725084942e-3_f64) * t38225;
    let t38228 = t6855 * t6262 * t10968;
    (t38190, t38211, t38226, t38228)
}
