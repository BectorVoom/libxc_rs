//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 561/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk561(t8189: f64, t2: f64, t7241: f64, t7760: f64, t82: f64, t7763: f64, t1542: f64, t17: f64, t9: f64) -> (f64, f64, f64, f64, f64) {
    let t8260 = 28.0_f64 / 27.0_f64 * t8189;
    let t8270 = t7241 * t2;
    let t8275 = t7760 * t82;
    let t8276 = t2 * t7763;
    let t8281 = t1542 * t17;
    let t8282 = t9 * t8281;
    (t8260, t8270, t8275, t8276, t8282)
}
