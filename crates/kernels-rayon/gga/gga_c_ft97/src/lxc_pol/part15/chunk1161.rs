//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1161/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1161(t41951: f64, t80770: f64, t80772: f64, t80819: f64, t80821: f64, t88221: f64, t88225: f64, t88229: f64, t89027: f64, t89030: f64, t89034: f64, t89038: f64, t89042: f64, t89047: f64) -> f64 {
    let t89727 = 4.0_f64 / 27.0_f64 * t80770 - 4.0_f64 / 27.0_f64 * t80772 + t41951 + 2.0_f64 / 9.0_f64 * t88221 - 2.0_f64 / 3.0_f64 * t88225 - 4.0_f64 / 9.0_f64 * t88229 - 2.0_f64 / 9.0_f64 * t80819 - 2.0_f64 / 9.0_f64 * t80821 - 10.0_f64 / 27.0_f64 * t89027 - 2.0_f64 * t89030 - 4.0_f64 / 3.0_f64 * t89034 - t89038 / 9.0_f64 + t89042 / 3.0_f64 - 40.0_f64 / 243.0_f64 * t89047;
    t89727
}
