//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1191/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1191<F: Float>(t1025: F, t16378: F, t16399: F, t6946: F, t5389: F, t621: F, t16421: F, t183: F, t2575: F, t568: F, t237: F, t5838: F, t1083: F, t5802: F, t7510: F, t1977: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20409 = t16378 * t1025;
    let t20436 = t16399 * t6946;
    let t20474 = t5389 * t621;
    let t20542 = t16421 * t183;
    let t20592 = t2575 * t568;
    let t20637 = t237 * t5838;
    let t20659 = t5802 * t1083;
    let t20663 = t237 * t7510;
    let t20671 = t237 * t1977;
    (t20409, t20436, t20474, t20542, t20592, t20637, t20659, t20663, t20671)
}
