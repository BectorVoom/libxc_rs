//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2365/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2365(t10214: f64, t10263: f64, t10390: f64, t10877: f64, t14130: f64, t14167: f64, t1539: f64, t2979: f64, t3048: f64, t3071: f64, t42380: f64, t42403: f64, t42412: f64, t43361: f64, t4562: f64, t4565: f64, t47689: f64, t47693: f64, t47720: f64, t47742: f64, t47767: f64, t973: f64, t977: f64) -> f64 {
    let t48543 = -t3048 * t14167 / 48.0_f64 - 11.0_f64 / 54.0_f64 * t10263 * t4562 + 11.0_f64 / 81.0_f64 * t10263 * t4565 + t42380 / 1152.0_f64 - t43361 * t3071 * t1539 * t10877 / 768.0_f64 - t42403 / 1152.0_f64 + t42412 / 2304.0_f64 - t10390 * t14130 / 768.0_f64 - t973 * t977 * t47767 / 144.0_f64 - 7.0_f64 / 54.0_f64 * t973 * t10214 * t47742 + t973 * t2979 * t47689 / 72.0_f64 + t973 * t2979 * t47693 / 72.0_f64 + 7.0_f64 / 216.0_f64 * t973 * t10214 * t47720;
    t48543
}
