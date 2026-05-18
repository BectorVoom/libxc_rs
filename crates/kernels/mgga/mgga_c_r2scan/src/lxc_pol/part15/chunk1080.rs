//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1080/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1080<F: Float>(t10868: F, t6535: F, t6536: F, t2116: F, t57: F, t6257: F, t261: F, t3304: F, t6457: F, t10879: F, t10891: F, t3299: F, t6470: F) -> (F, F, F, F, F) {
    let t38062 = t6535 * t10868 * t6536;
    let t38068 = t6257 * t57 * t2116;
    let t38069 = F::new(0.98171973930797904389e-1) * t38068;
    let t38074 = t3304 * t261 * t6457;
    let t38076 = t10879 * t10891;
    let t38079 = t3299 * t261 * t6470;
    (t38062, t38069, t38074, t38076, t38079)
}
