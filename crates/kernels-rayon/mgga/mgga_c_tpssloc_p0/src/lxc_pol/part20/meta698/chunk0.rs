//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2664/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2664(t157: f64, t54349: f64, t54372: f64, t17: f64, t184: f64, t39324: f64, t39327: f64, t39338: f64, t39346: f64, t39349: f64, t39356: f64, t54313: f64, t54315: f64, t54317: f64, t54318: f64, t54319: f64, t54320: f64, t54321: f64, t54322: f64, t54324: f64, t54326: f64) -> (f64, f64, f64) {
    let t54374 = (t54349 + t54372) * t157;
    let t54376 = t17 * t54374 * t184;
    let t54377 = -t39324 + t54313 - t54315 - t54317 + t39327 + t54318 + t54319 - t54320 - t39338 + t54321 - t54322 + t39346 + t39349 + t54324 + t39356 - t54326 + t54376;
    (t54374, t54376, t54377)
}
