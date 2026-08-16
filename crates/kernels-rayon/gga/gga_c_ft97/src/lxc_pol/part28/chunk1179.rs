//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1179/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1179(t26788: f64, t7309: f64, t1349: f64, t138438: f64, t138715: f64, t1969: f64, t23413: f64, t26551: f64, t26561: f64, t26809: f64, t26909: f64, t28: f64, t3052: f64, t32696: f64, t32709: f64, t32711: f64, t32722: f64, t33002: f64, t35022: f64, t3588: f64, t5772: f64, t5773: f64, t6580: f64, t7313: f64, t925: f64, t9432: f64) -> f64 {
    let t149398 = t7309 * t26788;
    let t149404 = 2.0_f64 * t5772 * t9432 * t5773 * t26909 - t26809 * t1969 * t32722 * t3052 / 9.0_f64 - t5772 * t1969 * t138438 * t925 / 18.0_f64 - t1349 * t28 * t32709 * t26551 / 3.0_f64 + t23413 * t35022 / 9.0_f64 - t1349 * t28 * t7313 * t3588 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t6580 * t33002 - t6580 * t32711 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t138715 + t149398 / 9.0_f64 + t6580 * t32696 / 3.0_f64 + t7309 * t26561 / 6.0_f64;
    t149404
}
