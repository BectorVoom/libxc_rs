//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 703/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk703(t375: f64, t6681: f64, t89: f64, t1017: f64, t23925: f64, t28: f64, t376: f64, t6677: f64, t26791: f64, t558: f64, t3408: f64, t5778: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27171 = t89 * t375 * t6681;
    let t27174 = t23925 * t1017;
    let t27175 = t28 * t27174;
    let t27176 = t89 * t27175;
    let t27178 = t376 * t6677;
    let t27179 = t89 * t27178;
    let t27181 = t26791 * t558;
    let t27182 = t28 * t27181;
    let t27183 = t89 * t27182;
    let t27185 = t5778 * t3408;
    (t27171, t27175, t27176, t27178, t27179, t27182, t27183, t27185)
}
