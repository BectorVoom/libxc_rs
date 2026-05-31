//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 207/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk207<F: Float>(t12: F, t2: F, t387: F, t390: F, t637: F) -> (F, F, F, F) {
    let t639 = F::cast_from(1.0_f64)/F::sqrt(t12);
    let t640 = t639 * t2;
    let t641 = t640 * t387;
    let t644 = F::cast_from(0.25319e1_f64) * t637 - F::cast_from(0.204775e0_f64) * t641 - F::cast_from(0.82156666666666666667e-1_f64) * t390;
    (t639, t640, t641, t644)
}
