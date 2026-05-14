//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 698/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk698<F: Float>(t4128: F, t5336: F, t5357: F, t5390: F, t105: F, t469: F, t96: F, t1670: F, t1674: F, t922: F, t1662: F, t814: F, t1679: F, t467: F, t301: F, t694: F) -> (F, F, F, F, F, F, F) {
    let t5392 = t4128 + t5336 + t5357 + t5390;
    let t5395 = t96 * t105 * t5392 * t469;
    let t5397 = t1674 * t1670 * t922;
    let t5399 = t1662 * t814;
    let t5401 = t1679 * t5399 * t467;
    let t5403 = t1662 * t469;
    let t5405 = t694 * t5403 * t301;
    (t5392, t5395, t5397, t5399, t5401, t5403, t5405)
}
