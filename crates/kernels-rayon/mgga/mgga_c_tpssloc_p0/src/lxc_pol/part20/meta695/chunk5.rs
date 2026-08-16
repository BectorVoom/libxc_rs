//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2652/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2652(t16288: f64, t3853: f64, t12384: f64, t5234: f64, t3795: f64, t40281: f64, t5293: f64, t12156: f64, t12397: f64, t12429: f64, t1363: f64, t16257: f64, t16271: f64, t16275: f64, t16278: f64, t16401: f64, t1799: f64, t1827: f64, t3858: f64, t39973: f64, t39975: f64, t39983: f64, t39989: f64, t40070: f64, t40119: f64, t5289: f64, t820: f64) -> f64 {
    let t54034 = t16288 * t3853;
    let t54042 = t5234 * t12384;
    let t54043 = t54042 * t3795;
    let t54047 = t40281 * t5293;
    let t54048 = 119.0_f64 / 4608.0_f64 * t54047;
    let t54058 = 35.0_f64 / 128.0_f64 * t1363 * t40070 * t820 * t1799 * t12156 + 7.0_f64 / 1536.0_f64 * t54034 - t16278 * t3858 / 1024.0_f64 - t40119 * t1827 / 3072.0_f64 - t12397 * t5289 / 1024.0_f64 - 7.0_f64 / 768.0_f64 * t54043 + 7.0_f64 / 1536.0_f64 * t39973 - 7.0_f64 / 768.0_f64 * t39983 - t54048 - t39975 * t5293 / 1024.0_f64 - t12429 * t16271 / 512.0_f64 - t12429 * t16275 / 1024.0_f64 + t16401 * t16257 / 256.0_f64 - 7.0_f64 / 384.0_f64 * t39989;
    t54058
}
