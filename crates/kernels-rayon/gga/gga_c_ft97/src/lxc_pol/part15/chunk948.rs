//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 948/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk948(t160: f64, t20655: f64, t20862: f64, t8392: f64, t20897: f64, t9438: f64, t20744: f64, t1882: f64, t20974: f64, t20630: f64, t549: f64, t20607: f64, t39922: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76567 = t160 * t20655;
    let t76607 = t8392 * t20862;
    let t76618 = t9438 * t20897;
    let t76623 = t8392 * t20744;
    let t76777 = t1882 * t20974;
    let t76876 = t549 * t20630;
    let t76899 = t39922 * t20607;
    (t76567, t76607, t76618, t76623, t76777, t76876, t76899)
}
