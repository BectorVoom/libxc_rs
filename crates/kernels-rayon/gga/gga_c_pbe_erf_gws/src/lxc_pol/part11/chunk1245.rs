//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1245/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1245(t11514: f64, t13408: f64, t2157: f64, t2343: f64, t3235: f64, t3247: f64, t3257: f64, t3803: f64, t3855: f64, t45421: f64, t45438: f64, t45450: f64, t45452: f64, t49568: f64, t49717: f64, t49722: f64, t49729: f64, t49730: f64, t6366: f64, t6685: f64) -> f64 {
    let t49736 = 7.0_f64 / 288.0_f64 * t45421 + 15.0_f64 / 64.0_f64 * t3247 * t6366 * t11514 * t49568 - t2343 * t3235 * t11514 * t3855 / 256.0_f64 - 7.0_f64 / 48.0_f64 * t45438 - t49717 - 5.0_f64 / 64.0_f64 * t2343 * t6366 * t11514 * t13408 + t49722 - 7.0_f64 / 48.0_f64 * t45450 + 7.0_f64 / 16.0_f64 * t45452 - t49729 + 3.0_f64 / 64.0_f64 * t6685 * t3257 * t3803 * t49730 * t2157;
    t49736
}
