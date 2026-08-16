//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2210/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2210(t10913: f64, t12595: f64, t12598: f64, t12606: f64, t12609: f64, t12612: f64, t1409: f64, t2244: f64, t2250: f64, t2291: f64, t2298: f64, t39096: f64, t39114: f64, t3966: f64, t4007: f64, t4012: f64, t45872: f64, t607: f64, t634: f64, t638: f64, t9258: f64, t9288: f64, t9321: f64, t9330: f64) -> f64 {
    let t45892 = 3640.0_f64 / 81.0_f64 * t39096 * t1409 * t9288 - 280.0_f64 / 9.0_f64 * t9321 * t3966 * t2244 - 280.0_f64 / 9.0_f64 * t12595 * t10913 + 28.0_f64 / 3.0_f64 * t2291 * t12606 * t607 + 28.0_f64 / 3.0_f64 * t12598 * t2250 + 28.0_f64 / 9.0_f64 * t4007 * t9258 - 4.0_f64 / 3.0_f64 * t634 * t45872 + 3640.0_f64 / 81.0_f64 * t39114 * t1409 * t9288 + 280.0_f64 / 9.0_f64 * t9330 * t3966 * t2244 + 280.0_f64 / 9.0_f64 * t12609 * t10913 + 28.0_f64 / 3.0_f64 * t2298 * t12606 * t607 + 28.0_f64 / 3.0_f64 * t12612 * t2250 + 28.0_f64 / 9.0_f64 * t4012 * t9258 + 4.0_f64 / 3.0_f64 * t638 * t45872;
    t45892
}
