//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 583/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk583(t419: f64, t8122: f64, t423: f64, t7745: f64, t420: f64, t1675: f64, t67: f64, t9: f64) -> (f64, f64, f64, f64, f64) {
    let t8123 = t419 * t8122;
    let t8125 = t423 * t7745;
    let t8126 = t420 * t8125;
    let t8127 = t419 * t8126;
    let t8130 = t9 * t67 * t1675;
    (t8123, t8125, t8126, t8127, t8130)
}
