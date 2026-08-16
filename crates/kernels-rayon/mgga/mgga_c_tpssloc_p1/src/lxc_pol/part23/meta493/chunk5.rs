//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1517/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1517(t119: f64, t16311: f64, t19876: f64, t20475: f64, t210: f64, t3733: f64, t40025: f64, t5246: f64, t5248: f64, t54151: f64, t56927: f64, t56946: f64, t56953: f64, t56993: f64, t57011: f64, t57019: f64, t57041: f64, t57073: f64, t74090: f64, t79921: f64, t80021: f64) -> f64 {
    let t80399 = 595.0_f64 / 2592.0_f64 * t54151 - 119.0_f64 / 2304.0_f64 * t56927 + 5.0_f64 / 4.0_f64 * t40025 * t210 * t119 * t80021 + 3.0_f64 / 16.0_f64 * t3733 * t210 * t119 * t79921 + 35.0_f64 / 12.0_f64 * t56946 - 35.0_f64 / 36.0_f64 * t56953 + 119.0_f64 / 288.0_f64 * t56993 + 595.0_f64 / 576.0_f64 * t57011 - 119.0_f64 / 576.0_f64 * t57019 + 119.0_f64 / 1152.0_f64 * t57041 - 119.0_f64 / 1152.0_f64 * t57073 + t5246 * t5248 * t74090 * t16311 / 384.0_f64 + t19876 * t20475 / 128.0_f64;
    t80399
}
