//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 376/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk376<F: Float>(t169: F, t3601: F, t299: F, t706: F, t3216: F, t3226: F, t3218: F, t3223: F, t3232: F, t3429: F, t471: F) -> (F, F, F, F, F) {
    let t3602 = t3601 * t169;
    let t3603 = t3602 * t299;
    let t3604 = t706 * t3603;
    let t3607 = 3.0 / 64.0 * t3216;
    let t3610 = t3226 / 64.0;
    let t3611 = t3607 - 9.0 / 2048.0 * t3218 + 3.0 / 2048.0 * t3223 - t3610;
    let t3614 = t3611 * t471 - 2.0 * t3232 + t3429 + t3607 - t3610;
    (t3602, t3603, t3604, t3611, t3614)
}
