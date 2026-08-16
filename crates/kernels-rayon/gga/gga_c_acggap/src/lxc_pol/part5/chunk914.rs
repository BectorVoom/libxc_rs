//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 914/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk914(t3244: f64, t993: f64, t13633: f64, t161: f64, t381: f64, t390: f64, t1077: f64, t368: f64, t384: f64, t398: f64, t879: f64, t1032: f64, t3732: f64) -> (f64, f64, f64, f64, f64) {
    let t13939 = 0.12862205435420921092e-2_f64 * t3244 * t993;
    let t13940 = t161 * t13633;
    let t13943 = 0.15117061203111996148e0_f64 * t381 * t13940 * t390;
    let t13949 = t384 * t398 * t368 * t879 * t1077;
    let t13951 = t1032 * t3732;
    (t13939, t13940, t13943, t13949, t13951)
}
