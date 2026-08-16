//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2033/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2033(t1284: f64, t17: f64, t9861: f64, t1287: f64, t9212: f64, t1285: f64, t9218: f64, t118: f64, t142: f64, t39283: f64) -> (f64, f64, f64, f64, f64) {
    let t39620 = t17 * t1284 * t9861;
    let t39634 = t9212 * t1287;
    let t39636 = t9212 * t1285;
    let t39655 = 480.0_f64 * t9218 * t1287;
    let t39658 = 0.11483599538271604938e-1_f64 * t118 * t39283 * t142;
    (t39620, t39634, t39636, t39655, t39658)
}
