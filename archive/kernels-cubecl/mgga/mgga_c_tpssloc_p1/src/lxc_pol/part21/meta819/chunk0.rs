//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2883/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2883<F: Float>(t136: F, t2826: F, t59668: F, t59672: F, t10304: F, t59725: F, t59755: F, t59746: F, t908: F, t4370: F, t896: F, t13634: F) -> (F, F, F, F, F, F, F) {
    let t60223 = t136 * t2826 * t59668;
    let t60226 = t136 * t2826 * t59672;
    let t60229 = t136 * t10304 * t59725;
    let t60232 = t136 * t2826 * t59755;
    let t60235 = t136 * t908 * t59746;
    let t60237 = t896 * t4370;
    let t60238 = t13634 * t60237;
    (t60223, t60226, t60229, t60232, t60235, t60237, t60238)
}
