//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1292/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1292<F: Float>(t331: F, t50539: F, t1076: F, t1162: F, t12041: F, t13662: F, t13678: F, t21823: F, t335: F, t338: F, t35003: F, t35188: F, t353: F, t3733: F, t3912: F, t39475: F, t39653: F, t39661: F, t46996: F, t47008: F, t47082: F, t47084: F, t47087: F, t47143: F, t6158: F, t859: F) -> F {
    let t50722 = t50539 * t331;
    let t50737 = t39475 * t13662 / F::new(16.0) + F::new(7.0) / F::new(12.0) * t46996 + t21823 + F::new(7.0) / F::new(12.0) * t47008 - t35003 * t859 * t353 * t1162 * t1076 / F::new(8.0) - F::new(7.0) / F::new(48.0) * t12041 * t35188 * t3733 - F::new(7.0) / F::new(48.0) * t3912 * t6158 * t50722 * t3733 + F::new(35.0) / F::new(72.0) * t39653 + F::new(35.0) / F::new(72.0) * t39661 + F::new(7.0) / F::new(36.0) * t47082 + F::new(7.0) / F::new(24.0) * t47084 - F::new(7.0) / F::new(12.0) * t47087 - t335 * t338 * t13678 * t1162 / F::new(24.0) + F::new(7.0) / F::new(6.0) * t47143;
    t50737
}
