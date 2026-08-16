//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1127/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1127<F: Float>(t10885: F, t11744: F, t10888: F, t30792: F, t11683: F, t22796: F, t10760: F, t25684: F, t6535: F, t20305: F, t24161: F, t25466: F) -> (F, F, F, F, F, F) {
    let t39522 = t11744 * t10885;
    let t39523 = F::cast_from(0.23115257973478049502e0_f64) * t39522;
    let t39524 = t30792 * t10888;
    let t39526 = t22796 * t11683;
    let t39529 = t6535 * t10760 * t25684;
    let t39532 = t20305 * t10760 * t24161;
    let t39535 = t6535 * t10760 * t25466;
    (t39523, t39524, t39526, t39529, t39532, t39535)
}
