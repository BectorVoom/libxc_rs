//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1060/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1060<F: Float>(t24019: F, t5290: F, t5289: F, t24014: F, t7430: F, t7429: F, t7316: F, t7315: F, t2532: F, t5283: F, t7317: F, t2575: F, t5320: F, t7312: F, t5278: F, t9030: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24251 = t5290 * t24019;
    let t24252 = t5289 * t24251;
    let t24254 = t7430 * t24014;
    let t24255 = t7429 * t24254;
    let t24257 = t7316 * t24014;
    let t24258 = t7315 * t24257;
    let t24260 = t5283 * t2532;
    let t24261 = t24260 * t7317;
    let t24263 = t2575 * t5320;
    let t24264 = t24263 * t7312;
    let t24266 = t5278 * t9030;
    (t24251, t24252, t24254, t24255, t24257, t24258, t24261, t24264, t24266)
}
