//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1057/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1057<F: Float>(t121661: F, t125260: F, t121656: F, t125268: F, t125279: F, t1493: F, t1925: F, t119457: F, t644: F, t1497: F, t8442: F, t640: F, t1469: F, t92669: F, t32591: F, t4186: F) -> (F, F, F, F, F, F, F, F) {
    let t128371 = t121661 * t125260;
    let t128374 = t121656 * t125268;
    let t128377 = t121656 * t125279;
    let t128401 = t1925 * t1493;
    let t128403 = t119457 * t128401 * t644;
    let t128409 = t1925 * t1497;
    let t128411 = t8442 * t128409 * t644;
    let t128415 = t119457 * t128409 * t640;
    let t128424 = t8442 * t92669 * t1469;
    let t128428 = t8442 * t32591 * t4186;
    (t128371, t128374, t128377, t128403, t128411, t128415, t128424, t128428)
}
