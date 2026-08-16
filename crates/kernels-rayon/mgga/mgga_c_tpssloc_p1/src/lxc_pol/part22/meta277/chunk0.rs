//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1426/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1426(t111: f64, t4025: f64, t1454: f64, t2281: f64, t4044: f64, t626: f64, t4068: f64, t2331: f64, t4067: f64, t2341: f64, t92: f64, t100: f64, t2349: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12725 = t4025 * t111;
    let t12747 = t2281 * t1454;
    let t12750 = 4.0_f64 / 3.0_f64 * t626 * t4044;
    let t12752 = 2.0_f64 / 3.0_f64 * t626 * t4068;
    let t12757 = t2331 * t4067;
    let t12774 = t92 * t2341;
    let t12795 = t100 * t2349;
    (t12725, t12747, t12750, t12752, t12757, t12774, t12795)
}
