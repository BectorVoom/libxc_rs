//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 732/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk732<F: Float>(t6045: F, t759: F, t122: F, t1415: F, t2111: F, t2117: F, t57: F, t1605: F, t537: F, t110: F, t1603: F, t2161: F) -> (F, F, F, F, F) {
    let t6047 = F::new(0.285764e-1) * t759 * t6045;
    let t6062 = F::cast_from(0.1590300183910403919e-2_f64) * t2111 * t122 * t1415 * t57 * t2117;
    let t6063 = t1605 * t537;
    let t6068 = t1603 * t110;
    let t6069 = t2161 * t6068;
    (t6047, t6062, t6063, t6068, t6069)
}
