//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 993/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk993<F: Float>(t2367: F, t6784: F, t6762: F, t8801: F, t4379: F, t810: F, t2074: F, t2182: F, t2352: F, t2373: F, t2376: F, t2388: F, t2392: F, t2408: F, t2409: F, t2417: F, t3066: F, t3207: F, t4402: F, t4491: F, t6107: F, t6760: F, t6781: F, t6802: F, t6822: F, t831: F, t8734: F, t9296: F) -> (F, F) {
    let t19974 = t2367 * t6784;
    let t19986 = t8801 * t6762;
    let t19993 = t4379 * t810;
    let t19998 = -3.0 / 8.0 * t3207 * t2409 * t2376 * t2182 * t2352 - 3.0 / 8.0 * t3066 * t2409 * t9296 * t2417 * t2352 + t2408 * t2409 * t6781 * t4491 / 4.0 + t3066 * t2409 * t8734 * t6822 / 4.0 - t2388 * t4402 / 16.0 + 7.0 / 12.0 * t19974 - t2392 * t4402 / 16.0 - t6802 * t2373 / 12.0 - t6107 * t2373 / 12.0 - 3.0 / 4.0 * t3207 * t2409 * t6781 * t6760 + 7.0 / 4.0 * t19986 + t2408 * t2409 * t2376 * t2074 * t2352 / 8.0 + t3207 * t2409 * t831 * t19993 / 4.0;
    (t19993, t19998)
}
