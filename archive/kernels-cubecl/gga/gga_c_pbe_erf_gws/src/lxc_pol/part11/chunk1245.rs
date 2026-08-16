//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1245/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1245<F: Float>(t11514: F, t13408: F, t2157: F, t2343: F, t3235: F, t3247: F, t3257: F, t3803: F, t3855: F, t45421: F, t45438: F, t45450: F, t45452: F, t49568: F, t49717: F, t49722: F, t49729: F, t49730: F, t6366: F, t6685: F) -> F {
    let t49736 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t45421 + F::cast_from(15.0_f64) / F::cast_from(64.0_f64) * t3247 * t6366 * t11514 * t49568 - t2343 * t3235 * t11514 * t3855 / F::cast_from(256.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t45438 - t49717 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t2343 * t6366 * t11514 * t13408 + t49722 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t45450 + F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t45452 - t49729 + F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t6685 * t3257 * t3803 * t49730 * t2157;
    t49736
}
