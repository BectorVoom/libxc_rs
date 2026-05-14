//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1348/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1348<F: Float>(t10028: F, t116096: F, t116098: F, t116101: F, t117258: F, t117260: F, t117262: F, t117265: F, t117267: F, t117434: F, t12345: F, t12352: F, t18923: F, t2049: F, t2666: F, t33151: F, t34386: F, t34612: F, t34615: F, t34618: F, t48504: F, t48510: F, t5532: F, t5533: F, t5552: F, t9760: F) -> (F,) {
    let t117548 = -6.0 * t10028 * t12352 * t5552 + 24.0 * t10028 * t48504 * t5533 - 12.0 * t12352 * t2049 * t34615 + 2.0 * t2666 * t33151 * t5532 + 4.0 * t12345 * t34612 + 4.0 * t12345 * t34615 + 4.0 * t12345 * t34618 - t18923 * t9760 - 12.0 * t34386 * t48510 - t116096 + t116098 - t116101 - t117258 - t117260 + t117262 - t117265 + t117267 + t117434;
    (t117548,)
}
