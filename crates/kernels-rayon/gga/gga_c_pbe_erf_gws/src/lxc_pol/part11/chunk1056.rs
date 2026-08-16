//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1056/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1056(t13473: f64, t2206: f64, t13545: f64, t20521: f64, t13171: f64, t2083: f64, t13126: f64, t20432: f64, t13553: f64, t2323: f64, t13375: f64, t13511: f64, t2319: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45546 = t2206 * t13473;
    let t45568 = t20521 * t13545;
    let t45574 = t13171 * t2083;
    let t45579 = t13126 * t20432;
    let t45582 = t2323 * t13553;
    let t45584 = t2206 * t13375;
    let t45620 = t2319 * t13511;
    (t45546, t45568, t45574, t45579, t45582, t45584, t45620)
}
