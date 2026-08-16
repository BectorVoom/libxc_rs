//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1054/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1054(t13254: f64, t6402: f64, t38143: f64, t9035: f64, t13271: f64, t13282: f64, t6484: f64, t11868: f64, t11984: f64, t13287: f64, t6416: f64, t13173: f64, t2119: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45345 = t6402 * t13254;
    let t45351 = t9035 * t38143;
    let t45353 = t6402 * t13271;
    let t45381 = t6484 * t13282;
    let t45400 = t11984 * t11868;
    let t45408 = t6416 * t13287;
    let t45410 = t13173 * t2119;
    (t45345, t45351, t45353, t45381, t45400, t45408, t45410)
}
