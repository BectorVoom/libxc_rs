//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 975/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk975<F: Float>(t2116: F, t57: F, t6257: F, t261: F, t3304: F, t6457: F, t10879: F, t10891: F, t3299: F, t6470: F, t10868: F, t2147: F, t6541: F, t6402: F, t10844: F, t10903: F, t2201: F) -> (F, F, F, F, F, F, F) {
    let t38068 = t6257 * t57 * t2116;
    let t38069 = 0.98171973930797904389e-1 * t38068;
    let t38074 = t3304 * t261 * t6457;
    let t38076 = t10879 * t10891;
    let t38079 = t3299 * t261 * t6470;
    let t38088 = t2147 * t10868 * t6541;
    let t38093 = t2147 * t10868 * t6402;
    let t38096 = t2201 * t10903 * t10844;
    (t38069, t38074, t38076, t38079, t38088, t38093, t38096)
}
