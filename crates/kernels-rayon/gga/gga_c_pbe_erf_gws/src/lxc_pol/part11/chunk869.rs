//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 869/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk869(t12187: f64, t13207: f64, t13212: f64, t13217: f64, t13223: f64, t13229: f64, t13609: f64, t13615: f64, t13619: f64, t13624: f64, t13628: f64, t13635: f64, t2408: f64, t3055: f64, t3066: f64, t335: f64, t3733: f64, t6731: f64, t6816: f64, t844: f64, t8818: f64, t9275: f64, t9290: f64, t9902: f64) -> f64 {
    let t13638 = -t9902 * t3733 / 32.0_f64 + t2408 * t13207 / 16.0_f64 - t3055 * t13212 / 32.0_f64 - t3055 * t13217 / 96.0_f64 - t6816 * t13223 / 4.0_f64 - 35.0_f64 / 144.0_f64 * t8818 + t2408 * t13229 / 16.0_f64 - t335 * t13609 / 96.0_f64 - t335 * t13615 / 16.0_f64 + t335 * t13619 / 16.0_f64 + t3066 * t13624 / 16.0_f64 - t335 * t13628 / 32.0_f64 - t6731 - 7.0_f64 / 16.0_f64 * t12187 + 35.0_f64 / 144.0_f64 * t9275 - 35.0_f64 / 72.0_f64 * t9290 - t844 * t13635 / 16.0_f64;
    t13638
}
