//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 621/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk621<F: Float>(t4939: F, t703: F, t1196: F, t284: F, t375: F, t5300: F, t89: F, t5226: F, t1882: F, t5214: F, t5225: F, t7640: F, t2336: F, t5221: F, t5217: F, t5209: F, t9725: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19168 = t703 * t4939;
    let t19233 = t1196 * t284;
    let t19246 = t89 * t375 * t5300;
    let t19249 = t89 * t375 * t5226;
    let t19278 = t1882 * t5214;
    let t19289 = t7640 * t5225;
    let t19298 = t89 * t2336 * t5221;
    let t19301 = t89 * t2336 * t5217;
    let t19304 = t89 * t9725 * t5209;
    (t19168, t19233, t19246, t19249, t19278, t19289, t19298, t19301, t19304)
}
