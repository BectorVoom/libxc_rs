//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1202/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1202(t10186: f64, t11296: f64, t12406: f64, t12899: f64, t12962: f64, t142: f64, t143: f64, t1503: f64, t169: f64, t19169: f64, t19174: f64, t19182: f64, t19203: f64, t19458: f64, t19466: f64, t2031: f64, t26477: f64, t26480: f64, t296: f64, t299: f64, t301: f64, t33446: f64, t34300: f64, t34326: f64, t3638: f64, t3671: f64, t42905: f64, t43168: f64, t48520: f64, t48741: f64, t48908: f64, t526: f64, t5651: f64, t8497: f64, t967: f64, t987: f64, t988: f64) -> f64 {
    let t48932 = -0.36991419282863461287e1_f64 * t26477 - 0.3486808982146430324e-2_f64 * t26480 - t988 * t2031 * t142 * t12962 + 18.0_f64 * t11296 * t10186 + t48908 * t296 - t19169 - t19174 - 0.47896936041018436376e-1_f64 * t43168 + t19182 + 0.20267214298646782767e-1_f64 * t169 * t299 * t48520 * t301 - 0.10931146159029059066e-3_f64 * t34300 + 18.0_f64 * t1503 * t143 * t48741 + 0.23948468020509218188e0_f64 * t34326 + 6.0_f64 * t988 * t33446 * t12406 + 24.0_f64 * t12899 * t987 * t526 - t19203 + 36.0_f64 * t42905 * t3638 - t19458 + t19466 - 12.0_f64 * t8497 * t5651 * t3671 * t967;
    t48932
}
