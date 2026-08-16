//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1052/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1052(t13220: f64, t6: f64, t254: f64, t1105: f64, t2407: f64, t3835: f64, t858: f64, t11564: f64, t8833: f64, t326: f64, t38036: f64, t13292: f64, t6183: f64, t9119: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45200 = t6 * t13220;
    let t45201 = t254 * t45200;
    let t45209 = t2407 * t858 * t3835 * t1105;
    let t45228 = t11564 * t8833;
    let t45235 = t326 * t38036;
    let t45240 = t9119 * t6183 * t13292;
    (t45200, t45201, t45209, t45228, t45235, t45240)
}
