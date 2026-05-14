//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1305/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1305<F: Float>(t2271: F, t9066: F, t3165: F, t6887: F, t3232: F, t6897: F, t797: F, t9563: F, t9069: F, t9072: F, t2321: F, t3128: F, t2333: F, t9056: F, t833: F, t9676: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31380 = t2271 * t9066;
    let t31388 = t6887 * t3165;
    let t31393 = t3232 * t6897;
    let t31402 = t9563 * t797;
    let t31444 = t2271 * t9069;
    let t31446 = t2271 * t9072;
    let t31459 = t2321 * t3128;
    let t31498 = t9563 * t2333;
    let t31514 = t2271 * t9056;
    let t31574 = t9676 * t833;
    (t31380, t31388, t31393, t31402, t31444, t31446, t31459, t31498, t31514, t31574)
}
