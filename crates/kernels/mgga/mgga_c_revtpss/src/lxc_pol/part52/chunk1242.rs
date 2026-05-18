//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1242/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1242<F: Float>(t13272: F, t32596: F, t8623: F, t32589: F, t121629: F, t34177: F, t1493: F, t1925: F, t119457: F, t644: F, t1497: F, t8442: F) -> (F, F, F, F, F, F) {
    let t128385 = t13272 * t32596 * t8623;
    let t128394 = t13272 * t32589;
    let t128399 = t121629 * t34177;
    let t128401 = t1925 * t1493;
    let t128403 = t119457 * t128401 * t644;
    let t128409 = t1925 * t1497;
    let t128411 = t8442 * t128409 * t644;
    (t128385, t128394, t128399, t128403, t128409, t128411)
}
