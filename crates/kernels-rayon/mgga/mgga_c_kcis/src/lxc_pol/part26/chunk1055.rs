//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1055/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1055(t2386: f64, t26579: f64, t7583: f64, t26576: f64, t7580: f64, t2140: f64, t3110: f64, t688: f64, t1075: f64, t2381: f64, t26434: f64, t26437: f64, t26441: f64, t26444: f64, t26448: f64, t26454: f64, t26572: f64, t26577: f64) -> (f64, f64) {
    let t26580 = t2386 * t26579;
    let t26581 = t26580 * t7583;
    let t26583 = t7580 * t26576;
    let t26586 = t688 * t3110 * t2140;
    let t26589 = t2381 * t1075 * t2140;
    let t26592 = 0.2653111111111111111e-1_f64 * t26434 + 0.99491666666666666664e-2_f64 * t26437 + 0.19898333333333333333e-1_f64 * t26441 - 0.19898333333333333333e-1_f64 * t26444 - 0.99491666666666666664e-2_f64 * t26448 - 0.13901041666666666667e-2_f64 * t26572 - 0.13901041666666666667e-2_f64 * t26577 - 0.43285526909722222222e-3_f64 * t26581 - 0.2782641015625e-3_f64 * t26583 - 0.5405960648148148148e-2_f64 * t26586 + 0.32435763888888888888e-2_f64 * t26589 - 0.2653111111111111111e-1_f64 * t26454;
    (t26580, t26592)
}
