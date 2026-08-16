//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 876/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk876(t1593: f64, t1655: f64, t1710: f64, t1712: f64, t3020: f64, t11360: f64, t1602: f64, t1685: f64, t35: f64, t428: f64, t11240: f64, t371: f64) -> (f64, f64, f64, f64, f64) {
    let t37960 = t1593 * t1655;
    let t37968 = t3020 * t1710 * t1712;
    let t37971 = t1602 * t11360;
    let t37977 = t35 * t1685;
    let t37978 = t37977 * t428;
    let t37985 = t371 * t11240;
    (t37960, t37968, t37971, t37978, t37985)
}
