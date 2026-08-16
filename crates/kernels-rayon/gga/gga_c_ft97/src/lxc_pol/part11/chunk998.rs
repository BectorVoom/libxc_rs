//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 998/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk998(t8392: f64, t9359: f64, t2133: f64, t582: f64, t1559: f64, t2075: f64, t2157: f64, t9124: f64, t2214: f64, t38953: f64, t9136: f64, t9363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40733 = t8392 * t9359;
    let t40735 = t582 * t2133;
    let t40739 = t1559 * t2075;
    let t40744 = t1559 * t2157;
    let t40749 = t8392 * t9124;
    let t40751 = t38953 * t2214;
    let t40753 = t8392 * t9136;
    let t40755 = t8392 * t9363;
    (t40733, t40735, t40739, t40744, t40749, t40751, t40753, t40755)
}
