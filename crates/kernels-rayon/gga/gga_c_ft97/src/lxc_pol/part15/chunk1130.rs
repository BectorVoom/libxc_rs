//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1130/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1130(t683: f64, t88756: f64, t92: f64, t88149: f64, t41446: f64, t88252: f64, t9568: f64, t88612: f64, t66202: f64, t80096: f64, t88737: f64, t88740: f64, t88744: f64, t88747: f64, t88751: f64, t88754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88758 = t92 * t683 * t88756;
    let t88761 = t92 * t683 * t88149;
    let t88764 = t41446 * t88252;
    let t88766 = t92 * t9568 * t88764;
    let t88769 = t92 * t9568 * t88612;
    let t88772 = -8.0_f64 * t88737 + 8.0_f64 * t88740 - 2.0_f64 / 3.0_f64 * t88744 - 8.0_f64 / 9.0_f64 * t88747 + 8.0_f64 * t88751 - 12.0_f64 * t88754 + 2.0_f64 * t88758 + 8.0_f64 / 3.0_f64 * t88761 - 8.0_f64 / 9.0_f64 * t66202 + 40.0_f64 / 9.0_f64 * t88766 - 20.0_f64 / 9.0_f64 * t88769 + 4.0_f64 / 9.0_f64 * t80096;
    (t88758, t88761, t88764, t88766, t88769, t88772)
}
