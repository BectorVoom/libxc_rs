//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1518/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1518(t12215: f64, t12351: f64, t1341: f64, t1343: f64, t1363: f64, t1799: f64, t1825: f64, t20416: f64, t20500: f64, t20565: f64, t210: f64, t3733: f64, t3790: f64, t3803: f64, t3870: f64, t40044: f64, t40168: f64, t5240: f64, t54582: f64, t57033: f64, t57310: f64, t57383: f64, t6330: f64, t6347: f64, t6370: f64, t6390: f64, t74592: f64, t80151: f64, t80181: f64, t80185: f64, t820: f64) -> f64 {
    let t80442 = 5.0_f64 / 32.0_f64 * t3803 * t40168 * t74592 * t1825 - 119.0_f64 / 2304.0_f64 * t57310 + t3733 * t210 * t20500 * t1799 / 4.0_f64 - t1341 * t1343 * t820 * t80151 / 3072.0_f64 + 5.0_f64 / 64.0_f64 * t5240 * t20565 - 15.0_f64 / 64.0_f64 * t1363 * t12351 * t820 * t6330 * t6347 + 7.0_f64 / 1536.0_f64 * t3790 * t1343 * t820 * t80181 + t57033 * t6390 / 256.0_f64 + 119.0_f64 / 2304.0_f64 * t57383 + 455.0_f64 / 162.0_f64 * t54582 - 3.0_f64 / 2.0_f64 * t12215 * t210 * t6370 * t6347 + 5.0_f64 / 192.0_f64 * t1363 * t3870 * t820 * t1799 * t20416 + t40044 * t1343 * t820 * t80185 / 128.0_f64;
    t80442
}
