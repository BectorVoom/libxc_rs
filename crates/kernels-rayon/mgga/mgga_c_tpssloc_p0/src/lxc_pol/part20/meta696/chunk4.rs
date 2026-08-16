//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2658/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2658(t40138: f64, t5303: f64, t12283: f64, t16366: f64, t16308: f64, t1352: f64, t16153: f64, t16224: f64, t16242: f64, t16311: f64, t3803: f64, t3805: f64, t3856: f64, t40052: f64, t40054: f64, t40060: f64, t40065: f64, t40079: f64, t40081: f64, t40083: f64, t40178: f64, t5246: f64, t5248: f64, t5249: f64, t54013: f64, t54015: f64) -> f64 {
    let t54220 = t40138 * t5303;
    let t54222 = t12283 * t16366;
    let t54237 = t12283 * t16308;
    let t54245 = -t3803 * t5248 * t16242 * t3856 / 1024.0_f64 - 7.0_f64 / 192.0_f64 * t54220 - 7.0_f64 / 192.0_f64 * t54222 + t3803 * t3805 * t5249 * t40178 / 768.0_f64 - 5.0_f64 / 256.0_f64 * t3803 * t16224 * t16153 * t1352 + 35.0_f64 / 384.0_f64 * t40052 + 3.0_f64 / 512.0_f64 * t5246 * t54013 * t16311 * t54015 - 7.0_f64 / 192.0_f64 * t54237 + 7.0_f64 / 384.0_f64 * t40054 + 595.0_f64 / 864.0_f64 * t40060 - 119.0_f64 / 1152.0_f64 * t40065 + 595.0_f64 / 1152.0_f64 * t40079 + 35.0_f64 / 192.0_f64 * t40081 - 35.0_f64 / 384.0_f64 * t40083;
    t54245
}
