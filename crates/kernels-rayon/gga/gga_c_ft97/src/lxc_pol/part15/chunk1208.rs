//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1208/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1208(t70142: f64, t83606: f64, t89772: f64, t89775: f64, t89781: f64, t89785: f64, t89789: f64, t89794: f64, t89798: f64, t89802: f64, t89807: f64, t89811: f64, t89815: f64) -> f64 {
    let t91158 = 2.0_f64 / 9.0_f64 * t89772 + 4.0_f64 / 9.0_f64 * t89775 + 4.0_f64 / 3.0_f64 * t89781 + 4.0_f64 / 3.0_f64 * t89785 + 4.0_f64 / 3.0_f64 * t89789 - t70142 - t89794 / 18.0_f64 - 4.0_f64 / 3.0_f64 * t89798 - t89802 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t83606 - 4.0_f64 / 3.0_f64 * t89807 + 4.0_f64 / 9.0_f64 * t89811 - 4.0_f64 / 3.0_f64 * t89815;
    t91158
}
