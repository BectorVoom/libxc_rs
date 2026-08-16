//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 831/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk831(t2719: f64, t788: f64, t2201: f64, t785: f64, t2202: f64, t2837: f64, t2139: f64, t2582: f64, t6062: f64, t6066: f64, t6073: f64, t6075: f64, t6084: f64, t6089: f64, t6095: f64, t7450: f64, t7454: f64, t7459: f64, t7461: f64, t7463: f64, t7468: f64, t7472: f64, t7475: f64) -> f64 {
    let t7476 = t788 * t2719;
    let t7479 = 0.11643651550782197811e-1_f64 * t2201 * t785 * t7476;
    let t7482 = 0.11643651550782197811e-1_f64 * t2201 * t2837 * t2202;
    let t7488 = 0.2600466522016280569e0_f64 * t2139 * t7450 - 0.86682217400542685632e-1_f64 * t2582 * t7454 - t7459 - 0.10401866088065122276e1_f64 * t7461 * t7463 - t7468 + t7472 - t7475 - t7479 - t7482 - t6062 + 0.19514881078765566037e-1_f64 * t6066 + 0.32524801797942610062e-3_f64 * t6073 + 0.12695991786046386926e-1_f64 * t6075 - t6084 + 0.11643651550782197811e-1_f64 * t6089 + 0.34930954652346593434e-1_f64 * t6095;
    t7488
}
