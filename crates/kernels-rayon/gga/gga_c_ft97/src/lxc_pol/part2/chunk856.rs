//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 856/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk856(t11717: f64, t3922: f64, t3936: f64, t458: f64, t2349: f64, t3690: f64) -> (f64, f64, f64) {
    let t13339 = t11717 * t3922;
    let t13345 = 2.0_f64 / 3.0_f64 * t458 * t3936;
    let t13346 = t3690 * t2349;
    (t13339, t13345, t13346)
}
