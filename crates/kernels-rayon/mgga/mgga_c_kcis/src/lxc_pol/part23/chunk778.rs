//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 778/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk778(t1444: f64, t461: f64, t543: f64, t3255: f64, t3773: f64, t1098: f64, t3783: f64, t3817: f64, t1479: f64, t3251: f64, t1484: f64, t3811: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11670 = 1.0_f64 / t461 / t1444;
    let t11671 = t11670 * t543;
    let t11690 = t3255 * t3773;
    let t11708 = t1098 * t3783;
    let t11710 = t1098 * t3817;
    let t11721 = t3251 * t1479;
    let t11723 = t3251 * t1484;
    let t11725 = t1098 * t3811;
    (t11670, t11671, t11690, t11708, t11710, t11721, t11723, t11725)
}
