//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1173/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1173(t10037: f64, t11296: f64, t11299: f64, t11300: f64, t12390: f64, t12395: f64, t12405: f64, t12422: f64, t12990: f64, t142: f64, t143: f64, t16394: f64, t16415: f64, t16418: f64, t16422: f64, t25965: f64, t26470: f64, t279: f64, t2858: f64, t2986: f64, t2990: f64, t33637: f64, t3620: f64, t3641: f64, t3686: f64, t42310: f64, t42325: f64, t42412: f64, t42842: f64, t475: f64, t48434: f64, t48528: f64, t48562: f64, t5651: f64, t8305: f64, t967: f64, t981: f64, t988: f64) -> f64 {
    let t48573 = 36.0_f64 * t8305 * t3641 * t11299 + 12.0_f64 * t2986 * t42842 - 3.0_f64 * t12990 * t3620 + 18.0_f64 * t2986 * t3641 * t10037 - 6.0_f64 * t988 * t16394 * t142 * t12405 * t981 + (t48434 + t48528) * t279 + 0.16271775250016674846e-1_f64 * t25965 + 12.0_f64 * t42412 * t2990 - 0.11622696607154767747e-2_f64 * t42310 + 12.0_f64 * t2986 * t12422 * t2858 + 24.0_f64 * t11296 * t12390 + 36.0_f64 * t33637 * t11300 - t16415 - t16418 + t16422 + 3.0_f64 * t475 * t143 * t48562 - 6.0_f64 * t3686 * t12395 - 0.21618361918556568284e0_f64 * t42325 + 24.0_f64 * t26470 * t5651 * t12405 * t967;
    t48573
}
