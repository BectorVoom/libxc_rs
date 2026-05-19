//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 188/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk188<F: Float>(t10: F, t128: F, t126: F, t69: F, t15: F, t24: F, t728: F, t97: F, t127: F, t129: F, t130: F, t60: F, t684: F) -> (F, F, F, F, F, F) {
    let t756 = t10 * t128;
    let t760 = t126 * t126;
    let t761 = t760 * t760;
    let t762 = t761 * t126;
    let t763 = t69 * t762;
    let t764 = t24 * t15;
    let t768 = t97 * t728;
    let t774 = F::cast_from(0.13140859333333333333e-2_f64) * t127 * t756 * t130 - F::cast_from(0.98556444999999999995e-3_f64) * t763 * t764 * t130 - F::cast_from(0.19711288999999999999e-2_f64) * t127 * t129 * t768 - F::new(4.0) * t60 * t684;
    (t756, t762, t763, t764, t768, t774)
}
