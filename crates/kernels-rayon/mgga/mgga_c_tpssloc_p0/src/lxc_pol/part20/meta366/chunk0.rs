//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1704/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1704(t2363: f64, t88: f64, t1454: f64, t2281: f64, t4044: f64, t626: f64, t4068: f64, t1453: f64, t2332: f64, t9365: f64, t2331: f64, t4067: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12739 = t88 * t2363;
    let t12747 = t2281 * t1454;
    let t12750 = 4.0_f64 / 3.0_f64 * t626 * t4044;
    let t12752 = 2.0_f64 / 3.0_f64 * t626 * t4068;
    let t12754 = t9365 * t1453 * t2332;
    let t12757 = t2331 * t4067;
    (t12739, t12747, t12750, t12752, t12754, t12757)
}
