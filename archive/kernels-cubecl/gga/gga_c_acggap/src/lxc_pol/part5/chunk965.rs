//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 965/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk965<F: Float>(t13092: F, t4419: F, t1005: F, t4523: F, t384: F, t398: F, t429: F, t4623: F, t4389: F, t4456: F, t174: F, t5079: F) -> (F, F, F, F, F) {
    let t15501 = t13092 * t4419;
    let t15508 = t1005 * t4523;
    let t15529 = t384 * t398 * t429 * t4623;
    let t15550 = t4389 * t4456;
    let t15560 = t174 * t5079;
    (t15501, t15508, t15529, t15550, t15560)
}
