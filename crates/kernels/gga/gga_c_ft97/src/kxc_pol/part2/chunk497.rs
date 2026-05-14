//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 497/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk497<F: Float>(t110: F, t3052: F, t447: F, t1882: F, t951: F, t3216: F, t3221: F, t3224: F, t3227: F, t3231: F, t3235: F, t3240: F, t3257: F, t3260: F, t3263: F, t3268: F, t3273: F, t3277: F, t3281: F, t446: F) -> (F, F) {
    let t3283 = t447 * t110 * t3052;
    let t3286 = t1882 * t951;
    let t3288 = t446 * t3216 / 3.0 + 2.0 / 3.0 * t446 * t3221 + t3224 / 9.0 - t446 * t3227 / 3.0 - t446 * t3231 / 3.0 - t446 * t3235 / 3.0 - t446 * t3240 / 3.0 - t446 * t3257 / 3.0 + t3260 / 9.0 - t446 * t3263 / 3.0 + 2.0 / 3.0 * t446 * t3268 + t446 * t3273 / 3.0 - t446 * t3277 / 9.0 - 2.0 / 9.0 * t3281 * t3283 + t3286 / 27.0;
    (t3283, t3288)
}
