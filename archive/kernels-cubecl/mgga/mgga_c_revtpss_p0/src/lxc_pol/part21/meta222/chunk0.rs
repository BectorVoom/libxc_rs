//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1326/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1326<F: Float>(t5351: F, t5352: F, t3720: F, t140: F, t1781: F, t1222: F, t127: F, t1789: F, t371: F, t1235: F, t1219: F, t1778: F) -> (F, F, F, F, F, F) {
    let t5353 = t5351 * t5352;
    let t5354 = t3720 * t5353;
    let t5357 = t140 * t1781;
    let t5358 = t1222 * t5357;
    let t5362 = t371 * t127 * t1789;
    let t5363 = t1235 * t5362;
    let t5366 = t1778 * t1219;
    (t5353, t5354, t5358, t5362, t5363, t5366)
}
