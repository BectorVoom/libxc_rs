//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1159/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1159(t89656: f64, t89684: f64, t66833: f64, t80677: f64, t80679: f64, t88143: f64, t88147: f64, t88151: f64, t88155: f64, t88159: f64, t88163: f64, t88167: f64, t88171: f64, t88178: f64, t88182: f64) -> (f64, f64) {
    let t89685 = t89656 + t89684;
    let t89704 = 20.0_f64 / 81.0_f64 * t88143 + 2.0_f64 / 9.0_f64 * t88147 + 4.0_f64 / 9.0_f64 * t88151 - 4.0_f64 / 27.0_f64 * t88155 - 2.0_f64 / 3.0_f64 * t88159 - 8.0_f64 / 9.0_f64 * t88163 + 4.0_f64 / 3.0_f64 * t88167 + 4.0_f64 / 3.0_f64 * t88171 + t66833 - 4.0_f64 / 9.0_f64 * t80677 + 4.0_f64 / 9.0_f64 * t80679 + 4.0_f64 / 3.0_f64 * t88178 + t88182 / 3.0_f64;
    (t89685, t89704)
}
