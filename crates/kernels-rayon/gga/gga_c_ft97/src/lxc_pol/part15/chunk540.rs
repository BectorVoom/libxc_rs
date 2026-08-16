//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 540/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk540(t2938: f64, t5457: f64, t898: f64, t2946: f64, t3738: f64, t4967: f64, t4971: f64, t4975: f64, t5242: f64, t5245: f64) -> (f64, f64) {
    let t5459 = t898 * t2938 * t5457;
    let t5468 = -0.117377e0_f64 * t5242 + 0.234754e0_f64 * t5245 + t2946 + 0.9628722222222222222e-1_f64 * t3738 - 0.9628722222222222222e-1_f64 * t4967 + 0.28886166666666666666e0_f64 * t4971 - 0.14443083333333333333e0_f64 * t4975;
    (t5459, t5468)
}
