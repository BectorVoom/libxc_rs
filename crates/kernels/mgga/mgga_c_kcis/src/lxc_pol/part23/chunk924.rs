//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 924/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk924<F: Float>(t2386: F, t26579: F, t7583: F, t26576: F, t7580: F, t2140: F, t3110: F, t688: F, t1075: F, t2381: F, t26434: F, t26437: F, t26441: F, t26444: F, t26448: F, t26454: F, t26572: F, t26577: F) -> (F, F) {
    let t26580 = t2386 * t26579;
    let t26581 = t26580 * t7583;
    let t26583 = t7580 * t26576;
    let t26586 = t688 * t3110 * t2140;
    let t26589 = t2381 * t1075 * t2140;
    let t26592 = 0.2653111111111111111e-1 * t26434 + 0.99491666666666666664e-2 * t26437 + 0.19898333333333333333e-1 * t26441 - 0.19898333333333333333e-1 * t26444 - 0.99491666666666666664e-2 * t26448 - 0.13901041666666666667e-2 * t26572 - 0.13901041666666666667e-2 * t26577 - 0.43285526909722222222e-3 * t26581 - 0.2782641015625e-3 * t26583 - 0.5405960648148148148e-2 * t26586 + 0.32435763888888888888e-2 * t26589 - 0.2653111111111111111e-1 * t26454;
    (t26580, t26592)
}
