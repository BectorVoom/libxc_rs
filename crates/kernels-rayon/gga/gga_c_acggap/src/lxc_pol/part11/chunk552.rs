//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 552/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk552(t3697: f64, t384: f64, t3669: f64, t381: f64, t390: f64, t1008: f64, t1020: f64, t1015: f64, t144: f64, t1077: f64, t322: f64, t368: f64, t398: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3699 = 0.21437009059034868486e-3_f64 * t384 * t3697;
    let t3700 = t381 * t3669;
    let t3702 = 0.17006693853500995666e-1_f64 * t3700 * t390;
    let t3703 = t1008 * t1020;
    let t3706 = 1.0_f64 / t1015 / t144;
    let t3730 = t1077 * t322;
    let t3732 = t398 * t368 * t3730;
    (t3699, t3702, t3703, t3706, t3730, t3732)
}
