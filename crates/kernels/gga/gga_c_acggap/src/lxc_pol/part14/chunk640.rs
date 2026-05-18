//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 640/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk640<F: Float>(t1165: F, t3457: F, t5852: F, t3456: F, t1772: F, t322: F, t368: F, t398: F, t384: F, t1795: F, t372: F, t1459: F) -> (F, F, F, F, F, F, F) {
    let t6184 = t1165 * t5852 * t3457;
    let t6185 = t3456 * t6184;
    let t6192 = t1772 * t322;
    let t6194 = t398 * t368 * t6192;
    let t6195 = t384 * t6194;
    let t6198 = t1795 * t372;
    let t6200 = t398 * t1459 * t6198;
    (t6184, t6185, t6192, t6194, t6195, t6198, t6200)
}
