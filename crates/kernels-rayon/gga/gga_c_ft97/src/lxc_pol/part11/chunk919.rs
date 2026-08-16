//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 919/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk919(t8392: f64, t8426: f64, t492: f64, t7765: f64, t1559: f64, t1588: f64, t432: f64, t1636: f64, t443: f64, t444: f64) -> (f64, f64, f64, f64, f64) {
    let t38935 = t8392 * t8426;
    let t38937 = t7765 * t492;
    let t38942 = t1559 * t1588;
    let t38947 = t7765 * t432;
    let t38953 = t443 * t444 * t1636;
    (t38935, t38937, t38942, t38947, t38953)
}
