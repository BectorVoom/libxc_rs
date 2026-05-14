//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1104/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1104<F: Float>(t19045: F, t324: F, t300: F, t6184: F, t983: F, t15547: F, t1642: F, t4719: F, t4725: F, t6104: F, t914: F, t936: F, t15416: F, t1610: F, t4590: F, t4632: F) -> (F, F, F, F, F, F, F, F) {
    let t19046 = t19045 * t324;
    let t19048 = 0.19751673498613801407e-1 * t300 * t19046;
    let t19049 = t300 * t6184;
    let t19051 = 0.5848223622634646207e0 * t19049 * t983;
    let t19053 = 0.11696447245269292414e1 * t15547 * t1642;
    let t19055 = 0.23392894490538584828e1 * t4719 * t4725;
    let t19056 = t6104 * t914;
    let t19058 = 1.0 * t19056 * t936;
    let t19060 = 2.0 * t15416 * t1610;
    let t19062 = 2.0 * t4590 * t4632;
    (t19046, t19048, t19051, t19053, t19055, t19058, t19060, t19062)
}
