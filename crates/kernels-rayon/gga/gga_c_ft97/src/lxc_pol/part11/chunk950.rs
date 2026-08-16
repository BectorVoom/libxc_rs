//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 950/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk950(t37315: f64, t446: f64, t569: f64, t2205: f64, t37320: f64, t1651: f64, t1986: f64, t9073: f64, t1882: f64, t9046: f64, t558: f64, t7966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39708 = t446 * t569 * t37315;
    let t39711 = t446 * t2205 * t37320;
    let t39713 = t1651 * t1986;
    let t39715 = t446 * t9073 * t39713;
    let t39717 = t1882 * t9046;
    let t39719 = t7966 * t558;
    (t39708, t39711, t39713, t39715, t39717, t39719)
}
