//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1161/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1161<F: Float>(t39911: F, t3308: F, t37652: F, t7379: F, t3295: F, t7509: F, t10708: F, t10710: F, t24912: F, t2183: F, t37754: F, t3602: F, t6087: F) -> (F, F, F, F, F) {
    let t39912 = F::cast_from(0.23115257973478049502e0_f64) * t39911;
    let t39914 = t37652 * t3308 * t7379;
    let t39916 = t3295 * t7509;
    let t39920 = t10708 * t10710 * t24912;
    let t39922 = t2183 * t37754;
    let t39924 = t39922 * t3602 * t6087;
    (t39912, t39914, t39916, t39920, t39924)
}
