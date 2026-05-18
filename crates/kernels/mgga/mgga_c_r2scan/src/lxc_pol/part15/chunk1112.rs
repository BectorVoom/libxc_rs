//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1112/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1112<F: Float>(t11670: F, t8098: F, t10710: F, t25480: F, t37658: F, t25486: F, t37582: F, t10776: F, t10810: F, t2563: F, t3308: F, t8102: F) -> (F, F, F, F, F) {
    let t39352 = t11670 * t8098;
    let t39355 = t37658 * t10710 * t25480;
    let t39358 = t37582 * t10710 * t25486;
    let t39361 = t10776 * t10810 * t2563;
    let t39362 = F::new(0.23115257973478049502e0) * t39361;
    let t39364 = t10776 * t3308 * t8102;
    (t39352, t39355, t39358, t39362, t39364)
}
