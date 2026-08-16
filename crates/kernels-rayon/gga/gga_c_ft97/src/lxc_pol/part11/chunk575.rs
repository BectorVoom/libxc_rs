//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 575/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk575(t1613: f64, t77: f64, t373: f64, t1608: f64, t384: f64, t39: f64, t1689: f64, t1691: f64, t1696: f64, t1609: f64, t1593: f64, t1632: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7998 = t77 * t1613;
    let t7999 = t7998 * t373;
    let t8000 = t1608 * t7999;
    let t8001 = t384 * t39;
    let t8002 = t1689 * t1691;
    let t8003 = t8002 * t1696;
    let t8007 = t77 * t1609;
    let t8008 = t8007 * t1593;
    let t8009 = t1608 * t8008;
    let t8010 = t1632 * t6;
    (t7998, t7999, t8000, t8001, t8002, t8003, t8007, t8008, t8009, t8010)
}
