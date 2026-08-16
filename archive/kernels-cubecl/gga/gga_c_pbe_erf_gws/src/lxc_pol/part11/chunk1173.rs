//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1173/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1173<F: Float>(t10037: F, t11296: F, t11299: F, t11300: F, t12390: F, t12395: F, t12405: F, t12422: F, t12990: F, t142: F, t143: F, t16394: F, t16415: F, t16418: F, t16422: F, t25965: F, t26470: F, t279: F, t2858: F, t2986: F, t2990: F, t33637: F, t3620: F, t3641: F, t3686: F, t42310: F, t42325: F, t42412: F, t42842: F, t475: F, t48434: F, t48528: F, t48562: F, t5651: F, t8305: F, t967: F, t981: F, t988: F) -> F {
    let t48573 = F::cast_from(36.0_f64) * t8305 * t3641 * t11299 + F::cast_from(12.0_f64) * t2986 * t42842 - F::cast_from(3.0_f64) * t12990 * t3620 + F::cast_from(18.0_f64) * t2986 * t3641 * t10037 - F::cast_from(6.0_f64) * t988 * t16394 * t142 * t12405 * t981 + (t48434 + t48528) * t279 + F::cast_from(0.16271775250016674846e-1_f64) * t25965 + F::cast_from(12.0_f64) * t42412 * t2990 - F::cast_from(0.11622696607154767747e-2_f64) * t42310 + F::cast_from(12.0_f64) * t2986 * t12422 * t2858 + F::cast_from(24.0_f64) * t11296 * t12390 + F::cast_from(36.0_f64) * t33637 * t11300 - t16415 - t16418 + t16422 + F::cast_from(3.0_f64) * t475 * t143 * t48562 - F::cast_from(6.0_f64) * t3686 * t12395 - F::cast_from(0.21618361918556568284e0_f64) * t42325 + F::cast_from(24.0_f64) * t26470 * t5651 * t12405 * t967;
    t48573
}
