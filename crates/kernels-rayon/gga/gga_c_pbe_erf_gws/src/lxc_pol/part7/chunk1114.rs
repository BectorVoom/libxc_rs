//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1114/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1114(t2367: f64, t6784: f64, t6762: f64, t8801: f64, t4379: f64, t810: f64, t2074: f64, t2182: f64, t2352: f64, t2373: f64, t2376: f64, t2388: f64, t2392: f64, t2408: f64, t2409: f64, t2417: f64, t3066: f64, t3207: f64, t4402: f64, t4491: f64, t6107: f64, t6760: f64, t6781: f64, t6802: f64, t6822: f64, t831: f64, t8734: f64, t9296: f64) -> (f64, f64) {
    let t19974 = t2367 * t6784;
    let t19986 = t8801 * t6762;
    let t19993 = t4379 * t810;
    let t19998 = -3.0_f64 / 8.0_f64 * t3207 * t2409 * t2376 * t2182 * t2352 - 3.0_f64 / 8.0_f64 * t3066 * t2409 * t9296 * t2417 * t2352 + t2408 * t2409 * t6781 * t4491 / 4.0_f64 + t3066 * t2409 * t8734 * t6822 / 4.0_f64 - t2388 * t4402 / 16.0_f64 + 7.0_f64 / 12.0_f64 * t19974 - t2392 * t4402 / 16.0_f64 - t6802 * t2373 / 12.0_f64 - t6107 * t2373 / 12.0_f64 - 3.0_f64 / 4.0_f64 * t3207 * t2409 * t6781 * t6760 + 7.0_f64 / 4.0_f64 * t19986 + t2408 * t2409 * t2376 * t2074 * t2352 / 8.0_f64 + t3207 * t2409 * t831 * t19993 / 4.0_f64;
    (t19993, t19998)
}
