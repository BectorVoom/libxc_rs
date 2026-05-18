//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1088/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1088<F: Float>(t122: F, t3434: F, t3437: F, t38213: F, t23194: F, t263: F, t3438: F, t3446: F, t6874: F, t10968: F, t6262: F, t6855: F) -> (F, F, F) {
    let t38220 = t3434 * t3437 * t38213 * t122;
    let t38225 = t3446 * t263 * t23194 * t3438 * t6874;
    let t38226 = F::new(0.91462949374725084942e-3) * t38225;
    let t38228 = t6855 * t6262 * t10968;
    (t38220, t38226, t38228)
}
