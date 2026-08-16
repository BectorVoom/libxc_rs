//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2679/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2679(t1799: f64, t5286: f64, t16224: f64, t16305: f64, t1825: f64, t19919: f64, t19924: f64, t3803: f64, t40006: f64, t40060: f64, t54063: f64, t57007: f64, t57009: f64, t57011: f64, t57019: f64, t57022: f64, t57041: f64, t57057: f64, t57071: f64, t57073: f64) -> (f64, f64) {
    let t74677 = t1799 * t5286;
    let t74682 = 455.0_f64 / 648.0_f64 * t40006 + 15.0_f64 / 128.0_f64 * t3803 * t54063 * t1825 * t19919 - 5.0_f64 / 128.0_f64 * t3803 * t16224 * t1825 * t19924 - 35.0_f64 / 384.0_f64 * t57007 + 7.0_f64 / 384.0_f64 * t57009 + 595.0_f64 / 1152.0_f64 * t57011 - 119.0_f64 / 1152.0_f64 * t57019 + 7.0_f64 / 384.0_f64 * t57022 + 595.0_f64 / 2592.0_f64 * t40060 + 119.0_f64 / 2304.0_f64 * t57041 + 7.0_f64 / 1536.0_f64 * t57057 - 7.0_f64 / 768.0_f64 * t57071 - 119.0_f64 / 2304.0_f64 * t57073 + t3803 * t16305 * t1825 * t74677 / 128.0_f64;
    (t74677, t74682)
}
