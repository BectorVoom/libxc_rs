//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 613/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk613<F: Float>(t1882: F, t2667: F, t2336: F, t2671: F, t89: F, t2680: F, t683: F, t191: F, t7640: F, t2683: F, t375: F, t793: F, t9733: F, t2675: F, t2661: F, t9725: F) -> (F, F, F, F, F, F, F, F) {
    let t10243 = t1882 * t2667;
    let t10246 = t89 * t2336 * t2671;
    let t10248 = t683 * t2680;
    let t10261 = t191 * t7640;
    let t10276 = t89 * t375 * t2683;
    let t10279 = t89 * t9733 * t793;
    let t10282 = t89 * t2336 * t2675;
    let t10286 = t89 * t9725 * t2661;
    (t10243, t10246, t10248, t10261, t10276, t10279, t10282, t10286)
}
