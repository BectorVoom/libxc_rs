//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 507/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk507(t221: f64, t3427: f64, t456: f64, t1176: f64, t135: f64, t1179: f64, t1174: f64, t1186: f64, t1089: f64, t405: f64, t974: f64, t337: f64, t51: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3428 = t221 * t3427;
    let t3430 = 0.18518518518518518518e-3_f64 * t456 * t3428;
    let t3431 = t135 * t1176;
    let t3432 = t3431 * t1179;
    let t3433 = t1174 * t3432;
    let t3435 = t135 * t1186;
    let t3436 = t1174 * t3435;
    let t3439 = 1.0_f64 / t405 / t1089;
    let t3440 = t974 * t3439;
    let t3446 = t51 * t337;
    (t3430, t3431, t3433, t3436, t3439, t3440, t3446)
}
