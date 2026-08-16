//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2679/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2679(t12251: f64, t12267: f64, t12429: f64, t16155: f64, t16233: f64, t16244: f64, t16405: f64, t3783: f64, t3805: f64, t40188: f64, t40190: f64, t40206: f64, t40282: f64, t5245: f64, t5252: f64, t5301: f64, t54556: f64, t54557: f64, t54561: f64, t54567: f64, t54582: f64) -> f64 {
    let t54584 = -t54556 + 7.0_f64 / 1536.0_f64 * t54557 + 5.0_f64 / 256.0_f64 * t3783 * t16155 - 7.0_f64 / 768.0_f64 * t54561 + 7.0_f64 / 192.0_f64 * t40188 - 7.0_f64 / 384.0_f64 * t40190 + 7.0_f64 / 256.0_f64 * t54567 + t12267 * t5245 * t5252 / 512.0_f64 - 5.0_f64 / 256.0_f64 * t12429 * t16405 + t16233 * t3805 * t5301 * t12251 / 128.0_f64 + 7.0_f64 / 4608.0_f64 * t40206 + t12429 * t16244 / 128.0_f64 + 119.0_f64 / 576.0_f64 * t40282 + 455.0_f64 / 648.0_f64 * t54582;
    t54584
}
