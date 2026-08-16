//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1668/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1668(t41690: f64, t51978: f64, t77736: f64, t88118: f64, t88126: f64, t88134: f64, t88168: f64, t88171: f64, t88203: f64, t88206: f64, t88209: f64, t88211: f64, t88214: f64, t88216: f64) -> f64 {
    let t88305 = 0.250068e1_f64 * t88168 + 0.62517e0_f64 * t88171 + t41690 + 0.166712e1_f64 * t77736 + 0.21424148148148148148e1_f64 * t51978 - 0.34431666666666666667e1_f64 * t88118 + 0.123954e2_f64 * t88126 - 0.13772666666666666667e1_f64 * t88134 + 0.3529725e1_f64 * t88203 - 0.6618234375e1_f64 * t88206 - 0.13892666666666666667e0_f64 * t88209 + 0.2366859375e0_f64 * t88211 + 0.94674375e0_f64 * t88214 - 0.52945875e1_f64 * t88216;
    t88305
}
