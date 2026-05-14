//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1179/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1179<F: Float>(t1150: F, t20447: F, t1131: F, t12243: F, t6474: F, t3531: F, t6548: F, t12297: F, t12382: F, t16706: F, t16708: F, t16797: F, t16798: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> (F, F, F, F) {
    let t20448 = t20447 * t1150;
    let t20450 = 1.0 * t1131 * t20448;
    let t20452 = 0.16081979498692535067e2 * t12243 * t6474;
    let t20454 = 0.11696447245269292414e1 * t3531 * t6548;
    let t20469 = -t12382 + 0.79148148148148148147e-2 * t12297 + 0.15829629629629629629e-1 * t16706 + 0.79148148148148148147e-2 * t16708 - t16797 - t16798 + 0.39574074074074074073e-2 * t20283 + 0.19787037037037037037e-1 * t20295 - 0.71233333333333333332e-1 * t20300 - 0.23744444444444444444e-1 * t20304 - 0.11872222222222222222e-1 * t20285 + 0.10685e0 * t20308 + 0.71233333333333333332e-1 * t20312 - 0.5936111111111111111e-2 * t20287 - 0.11872222222222222222e-1 * t20315 + 0.35616666666666666666e-1 * t20320 + 0.17808333333333333333e-1 * t20290;
    (t20450, t20452, t20454, t20469)
}
