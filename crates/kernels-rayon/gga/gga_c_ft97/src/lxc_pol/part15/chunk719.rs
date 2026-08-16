//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 719/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk719(t1969: f64, t20556: f64, t446: f64, t4714: f64, t925: f64, t4668: f64, t9073: f64, t1017: f64, t4458: f64, t12571: f64, t20536: f64, t20540: f64, t20543: f64, t20547: f64, t20551: f64, t20554: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20557 = t1969 * t20556;
    let t20558 = t446 * t20557;
    let t20560 = t925 * t4714;
    let t20561 = t1969 * t20560;
    let t20562 = t446 * t20561;
    let t20564 = t925 * t4668;
    let t20565 = t9073 * t20564;
    let t20566 = t446 * t20565;
    let t20568 = t4458 * t1017;
    let t20569 = t1969 * t20568;
    let t20570 = t446 * t20569;
    let t20573 = -5.0_f64 / 81.0_f64 * t20536 - t20540 / 3.0_f64 + t20543 / 3.0_f64 + t20547 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t20551 - t20554 / 9.0_f64 + t20558 / 6.0_f64 + t20562 / 6.0_f64 - t20566 / 3.0_f64 - t20570 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t12571;
    (t20557, t20558, t20560, t20561, t20562, t20564, t20565, t20566, t20568, t20569, t20570, t20573)
}
