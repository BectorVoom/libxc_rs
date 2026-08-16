//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 954/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk954(t1647: f64, t1986: f64, t446: f64, t9073: f64, t2075: f64, t1985: f64, t27: f64, t89: f64, t1555: f64, t37357: f64, t9025: f64, t143: f64, t37355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39765 = t1647 * t1986;
    let t39767 = t446 * t9073 * t39765;
    let t39769 = t2075 * t2075;
    let t39772 = t89 * t27 * t1985 * t39769;
    let t39776 = t89 * t1555 * t9025 * t37357;
    let t39778 = t143 * t37355;
    (t39765, t39767, t39769, t39772, t39776, t39778)
}
