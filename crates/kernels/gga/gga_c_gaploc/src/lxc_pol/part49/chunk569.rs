//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 569/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk569<F: Float>(t10148: F, t10181: F, t10230: F, t10279: F, t3362: F, t501: F, t3366: F, t605: F, t2902: F, t921: F, t1016: F, t2497: F, t3418: F, t3381: F, t4379: F, t2366: F, t2754: F) -> (F, F, F, F, F, F, F, F) {
    let t10281 = t10148 + t10181 + t10230 + t10279;
    let t10283 = t3362 * t501;
    let t10295 = t3366 * t605;
    let t10298 = t2902 * t921;
    let t10301 = t1016 * t2497;
    let t10305 = t3418 * t605;
    let t10308 = t4379 * t3381;
    let t10309 = 0.14896037479937677779e-1 * t10308;
    let t10310 = t2366 * t2754;
    (t10281, t10283, t10295, t10298, t10301, t10305, t10309, t10310)
}
