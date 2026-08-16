//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1518/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1518<F: Float>(t12215: F, t12351: F, t1341: F, t1343: F, t1363: F, t1799: F, t1825: F, t20416: F, t20500: F, t20565: F, t210: F, t3733: F, t3790: F, t3803: F, t3870: F, t40044: F, t40168: F, t5240: F, t54582: F, t57033: F, t57310: F, t57383: F, t6330: F, t6347: F, t6370: F, t6390: F, t74592: F, t80151: F, t80181: F, t80185: F, t820: F) -> F {
    let t80442 = F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t3803 * t40168 * t74592 * t1825 - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t57310 + t3733 * t210 * t20500 * t1799 / F::cast_from(4.0_f64) - t1341 * t1343 * t820 * t80151 / F::cast_from(3072.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t5240 * t20565 - F::cast_from(15.0_f64) / F::cast_from(64.0_f64) * t1363 * t12351 * t820 * t6330 * t6347 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t3790 * t1343 * t820 * t80181 + t57033 * t6390 / F::cast_from(256.0_f64) + F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t57383 + F::cast_from(455.0_f64) / F::cast_from(162.0_f64) * t54582 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12215 * t210 * t6370 * t6347 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t1363 * t3870 * t820 * t1799 * t20416 + t40044 * t1343 * t820 * t80185 / F::cast_from(128.0_f64);
    t80442
}
