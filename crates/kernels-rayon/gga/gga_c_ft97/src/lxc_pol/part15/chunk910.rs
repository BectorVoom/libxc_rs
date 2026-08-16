//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 910/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk910(t4664: f64, t7773: f64, t89: f64, t37345: f64, t4652: f64, t4660: f64, t61462: f64, t62134: f64, t4759: f64, t8282: f64, t4765: f64, t4762: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t62287 = t89 * t7773 * t4664;
    let t62309 = t89 * t37345 * t4652;
    let t62317 = t89 * t7773 * t4660;
    let t62364 = 8.0_f64 / 27.0_f64 * t61462;
    let t62410 = 8.0_f64 / 9.0_f64 * t62134;
    let t62587 = t8282 * t4759;
    let t62599 = t8282 * t4765;
    let t62629 = t8282 * t4762;
    (t62287, t62309, t62317, t62364, t62410, t62587, t62599, t62629)
}
