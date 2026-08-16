//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2167/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2167(t26322: f64, t7708: f64, t91202: f64, t20004: f64, t26309: f64, t19945: f64, t19981: f64, t22833: f64, t19994: f64, t221: f64, t26284: f64, t19631: f64, t1998: f64, t236: f64, t6926: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97410 = t91202 * t7708 * t26322;
    let t97412 = t26309 * t20004;
    let t97414 = t26309 * t19945;
    let t97416 = t22833 * t19981;
    let t97419 = t26284 * t221 * t19994;
    let t97423 = t6926 * t1998 * t236 * t19631;
    (t97410, t97412, t97414, t97416, t97419, t97423)
}
