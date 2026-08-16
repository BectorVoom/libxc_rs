//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 740/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk740(t32152: f64, t5572: f64, t1293: f64, t38: f64, t1711: f64, t64: f64, t378: f64, t53: f64, t171: f64, t5555: f64, t39: f64, t8051: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32153 = t32152 * t5572;
    let t32156 = t38 * t1293;
    let t32161 = t64 * t1711;
    let t32163 = t378 * t53;
    let t32164 = t5555 * t171 * t32163;
    let t32167 = t8051 * t39;
    (t32153, t32156, t32161, t32163, t32164, t32167)
}
