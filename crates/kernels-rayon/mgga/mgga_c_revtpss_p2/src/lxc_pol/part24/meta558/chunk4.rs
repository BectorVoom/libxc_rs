//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1673/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1673(t41610: f64, t51978: f64, t77736: f64, t88118: f64, t88126: f64, t88134: f64, t88168: f64, t88171: f64, t88203: f64, t88206: f64, t88209: f64, t88211: f64, t88214: f64, t88216: f64) -> f64 {
    let t88396 = 0.197176e1_f64 * t88168 + 0.49293999999999999999e0_f64 * t88171 + t41610 + 0.13145066666666666666e1_f64 * t77736 + 0.12401580246913580247e1_f64 * t51978 - 0.19931111111111111111e1_f64 * t88118 + 0.71752000000000000001e1_f64 * t88126 - 0.79724444444444444444e0_f64 * t88134 + 0.1898925e1_f64 * t88203 - 0.3560484375e1_f64 * t88206 - 0.10954222222222222222e0_f64 * t88209 + 0.1151859375e0_f64 * t88211 + 0.46074375e0_f64 * t88214 - 0.28483875e1_f64 * t88216;
    t88396
}
