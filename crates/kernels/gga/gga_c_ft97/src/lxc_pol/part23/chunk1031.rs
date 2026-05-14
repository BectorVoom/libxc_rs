//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1031/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1031<F: Float>(t4246: F, t7124: F, t25188: F, t5309: F, t25140: F, t4917: F, t2665: F, t446: F, t4635: F, t6334: F, t25037: F, t10409: F, t28533: F, t992: F, t231: F, t5295: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31358 = t4246 * t7124;
    let t31360 = t25188 * t5309;
    let t31362 = t25140 * t4917;
    let t31363 = t2665 * t31362;
    let t31364 = t446 * t31363;
    let t31366 = t6334 * t4635;
    let t31367 = t2665 * t31366;
    let t31368 = t446 * t31367;
    let t31370 = t25037 * t4917;
    let t31371 = t10409 * t31370;
    let t31372 = t446 * t31371;
    let t31374 = t28533 * t992;
    let t31375 = t2665 * t31374;
    let t31376 = t446 * t31375;
    let t31381 = t231 * t5295;
    (t31358, t31360, t31363, t31364, t31367, t31368, t31371, t31372, t31375, t31376, t31381)
}
