//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 831/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk831<F: Float>(t2719: F, t788: F, t2201: F, t785: F, t2202: F, t2837: F, t2139: F, t2582: F, t6062: F, t6066: F, t6073: F, t6075: F, t6084: F, t6089: F, t6095: F, t7450: F, t7454: F, t7459: F, t7461: F, t7463: F, t7468: F, t7472: F, t7475: F) -> F {
    let t7476 = t788 * t2719;
    let t7479 = F::cast_from(0.11643651550782197811e-1_f64) * t2201 * t785 * t7476;
    let t7482 = F::cast_from(0.11643651550782197811e-1_f64) * t2201 * t2837 * t2202;
    let t7488 = F::cast_from(0.2600466522016280569e0_f64) * t2139 * t7450 - F::cast_from(0.86682217400542685632e-1_f64) * t2582 * t7454 - t7459 - F::cast_from(0.10401866088065122276e1_f64) * t7461 * t7463 - t7468 + t7472 - t7475 - t7479 - t7482 - t6062 + F::cast_from(0.19514881078765566037e-1_f64) * t6066 + F::cast_from(0.32524801797942610062e-3_f64) * t6073 + F::cast_from(0.12695991786046386926e-1_f64) * t6075 - t6084 + F::cast_from(0.11643651550782197811e-1_f64) * t6089 + F::cast_from(0.34930954652346593434e-1_f64) * t6095;
    t7488
}
