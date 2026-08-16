//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 988/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk988(t4044: f64, t626: f64, t4068: f64, t1453: f64, t2332: f64, t9365: f64, t2331: f64, t4067: f64, t666: f64, t2358: f64, t4043: f64, t1444: f64, t2342: f64, t9384: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12750 = 4.0_f64 / 3.0_f64 * t626 * t4044;
    let t12752 = 2.0_f64 / 3.0_f64 * t626 * t4068;
    let t12754 = t9365 * t1453 * t2332;
    let t12757 = t2331 * t4067;
    let t12758 = t12757 * t666;
    let t12761 = t4043 * t2358;
    let t12771 = t9384 * t1444 * t2342;
    (t12750, t12752, t12754, t12758, t12761, t12771)
}
