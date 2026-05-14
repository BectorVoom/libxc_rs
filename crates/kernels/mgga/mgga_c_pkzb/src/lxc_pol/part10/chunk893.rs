//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 893/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk893<F: Float>(t326: F, t6523: F, t2370: F, t5728: F, t2099: F, t2389: F, t918: F, t941: F, t2363: F, t937: F, t2970: F, t6417: F, t2421: F, t914: F, t2393: F, t1444: F, t42: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6524 = t6523 * t326;
    let t6526 = t5728 * t2370;
    let t6531 = t2099 * t2389;
    let t6532 = t918 * t6531;
    let t6545 = t941 * t941;
    let t6546 = 1.0 / t6545;
    let t6561 = t2363 * t937;
    let t6566 = t2970 * t6417;
    let t6574 = t914 * t2421;
    let t6579 = t2393 * t937;
    let t6631 = t1444 * t42;
    (t6524, t6526, t6531, t6532, t6545, t6546, t6561, t6566, t6574, t6579, t6631)
}
