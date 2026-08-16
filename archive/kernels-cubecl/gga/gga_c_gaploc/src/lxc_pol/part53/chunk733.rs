//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 733/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk733<F: Float>(t13124: F, t13132: F, t13138: F, t13140: F, t13144: F, t13152: F, t13156: F, t13160: F, t13163: F, t13901: F, t13904: F, t13906: F) -> F {
    let t14515 = F::cast_from(0.14300195980740170668e1_f64) * t13901 - F::cast_from(0.21450293971110256002e1_f64) * t13904 + F::cast_from(0.71500979903700853338e0_f64) * t13906 + t13124 - t13132 + t13138 + t13140 + t13144 - t13152 + t13156 - t13160 - t13163;
    t14515
}
