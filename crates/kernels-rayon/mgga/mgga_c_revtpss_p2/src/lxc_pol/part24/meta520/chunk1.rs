//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1548/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1548(t1811: f64, t20849: f64, t6564: f64, t1770: f64, t6695: f64, t12772: f64, t24568: f64, t5340: f64, t24572: f64, t5331: f64, t11249: f64, t24543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82204 = t20849 * t1811;
    let t82217 = t6564 * t1811;
    let t82238 = t1770 * t6695;
    let t82286 = t5340 * t12772 * t24568;
    let t82289 = t5331 * t12772 * t24572;
    let t82293 = t24543 * t11249;
    (t82204, t82217, t82238, t82286, t82289, t82293)
}
