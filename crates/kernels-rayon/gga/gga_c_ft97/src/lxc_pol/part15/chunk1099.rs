//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1099/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1099(t16832: f64, t20049: f64, t3359: f64, t39546: f64, t4466: f64, t59002: f64, t59007: f64, t73975: f64, t73977: f64, t73983: f64, t73985: f64, t85522: f64, t85529: f64, t85536: f64, t85544: f64, t85551: f64) -> f64 {
    let t88010 = t39546 - 0.77029777777777777776e0_f64 * t73975 + 0.11554466666666666666e1_f64 * t73977 - 0.51995099999999999998e1_f64 * t85544 + 0.11554466666666666666e1_f64 * t85551 - 0.9628722222222222222e0_f64 * t85522 + 0.34663399999999999999e1_f64 * t85529 - 0.38514888888888888888e0_f64 * t85536 + 0.21397160493827160493e0_f64 * t73983 + 0.19257444444444444444e0_f64 * t73985 + 0.1056393e1_f64 * t16832 * t4466 - 0.469508e0_f64 * t3359 * t20049 - 0.25676592592592592592e0_f64 * t59002 + 0.77029777777777777776e0_f64 * t59007;
    t88010
}
