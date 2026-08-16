//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2886/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2886<F: Float>(t17271: F, t2815: F, t896: F, t17210: F, t2807: F, t13615: F, t4362: F, t17215: F, t17218: F, t17255: F, t699: F, t136: F, t59730: F, t908: F) -> (F, F, F, F, F, F, F) {
    let t60263 = t2815 * t17271 * t896;
    let t60265 = t17210 * t2807;
    let t60267 = t4362 * t13615;
    let t60269 = t17215 * t2807;
    let t60271 = t17218 * t2807;
    let t60274 = t699 * t17255;
    let t60277 = t136 * t908 * t59730;
    (t60263, t60265, t60267, t60269, t60271, t60274, t60277)
}
