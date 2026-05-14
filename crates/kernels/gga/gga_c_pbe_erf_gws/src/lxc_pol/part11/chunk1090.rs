//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1090/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1090<F: Float>(t11459: F, t13408: F, t2168: F, t6523: F, t45444: F, t1105: F, t13291: F, t2147: F, t337: F, t9119: F, t3824: F, t816: F, t11514: F, t2157: F, t2343: F, t3235: F, t3247: F, t3257: F, t3803: F, t3855: F, t45421: F, t45438: F, t45450: F, t45452: F, t49568: F, t6366: F, t6685: F) -> (F, F, F, F, F) {
    let t49717 = 3.0 / 8.0 * t2168 * t6523 * t11459 * t13408;
    let t49722 = 7.0 / 24.0 * t45444;
    let t49729 = t9119 * t2147 * t337 * t13291 * t1105 / 6.0;
    let t49730 = t816 * t3824;
    let t49736 = 7.0 / 288.0 * t45421 + 15.0 / 64.0 * t3247 * t6366 * t11514 * t49568 - t2343 * t3235 * t11514 * t3855 / 256.0 - 7.0 / 48.0 * t45438 - t49717 - 5.0 / 64.0 * t2343 * t6366 * t11514 * t13408 + t49722 - 7.0 / 48.0 * t45450 + 7.0 / 16.0 * t45452 - t49729 + 3.0 / 64.0 * t6685 * t3257 * t3803 * t49730 * t2157;
    (t49717, t49722, t49729, t49730, t49736)
}
