//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1168/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1168(t2665: f64, t446: f64, t89813: f64, t70141: f64, t83606: f64, t83619: f64, t89772: f64, t89775: f64, t89778: f64, t89781: f64, t89785: f64, t89789: f64, t89794: f64, t89798: f64, t89802: f64, t89807: f64, t89811: f64) -> (f64, f64) {
    let t89815 = t446 * t2665 * t89813;
    let t89818 = 4.0_f64 / 3.0_f64 * t89772 + 8.0_f64 / 3.0_f64 * t89775 - t89778 + 8.0_f64 * t89781 + 8.0_f64 * t89785 + 8.0_f64 * t89789 - 8.0_f64 / 3.0_f64 * t70141 - t89794 / 3.0_f64 - 8.0_f64 * t89798 - 2.0_f64 / 3.0_f64 * t89802 + 4.0_f64 / 9.0_f64 * t83606 - 8.0_f64 * t89807 + 8.0_f64 / 3.0_f64 * t89811 - 8.0_f64 * t89815 + 8.0_f64 / 3.0_f64 * t83619;
    (t89815, t89818)
}
