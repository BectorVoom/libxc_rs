//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 869/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk869<F: Float>(t12187: F, t13207: F, t13212: F, t13217: F, t13223: F, t13229: F, t13609: F, t13615: F, t13619: F, t13624: F, t13628: F, t13635: F, t2408: F, t3055: F, t3066: F, t335: F, t3733: F, t6731: F, t6816: F, t844: F, t8818: F, t9275: F, t9290: F, t9902: F) -> F {
    let t13638 = -t9902 * t3733 / F::cast_from(32.0_f64) + t2408 * t13207 / F::cast_from(16.0_f64) - t3055 * t13212 / F::cast_from(32.0_f64) - t3055 * t13217 / F::cast_from(96.0_f64) - t6816 * t13223 / F::cast_from(4.0_f64) - F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t8818 + t2408 * t13229 / F::cast_from(16.0_f64) - t335 * t13609 / F::cast_from(96.0_f64) - t335 * t13615 / F::cast_from(16.0_f64) + t335 * t13619 / F::cast_from(16.0_f64) + t3066 * t13624 / F::cast_from(16.0_f64) - t335 * t13628 / F::cast_from(32.0_f64) - t6731 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t12187 + F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t9275 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t9290 - t844 * t13635 / F::cast_from(16.0_f64);
    t13638
}
