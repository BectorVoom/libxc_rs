//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 947/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk947(t119: f64, t20356: f64, t210: f64, t1810: f64, t6347: f64, t11982: f64, t11984: f64, t20354: f64, t20355: f64, t20360: f64, t20361: f64, t20365: f64, t20366: f64, t20370: f64, t9457: f64, t9476: f64, t9484: f64) -> (f64, f64, f64) {
    let t20511 = t119 * t20356;
    let t20512 = t210 * t20511;
    let t20516 = t210 * t1810 * t6347;
    let t20519 = -t20354 - t9457 + t20355 + t9476 + t9484 - t20360 - t20361 + t11982 - t20365 - t20366 - t11984 - t20370;
    (t20512, t20516, t20519)
}
