//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1006/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1006(t331: f64, t3379: f64, t551: f64, t553: f64, t3380: f64, t547: f64, t11162: f64, t164: f64, t5969: f64, t5977: f64, t5982: f64, t5986: f64, t5988: f64, t5990: f64, t5993: f64, t5996: f64, t5999: f64, t8471: f64, t8474: f64, t8477: f64, t8478: f64, t8489: f64, t8490: f64) -> f64 {
    let t11262 = t331 * t3379;
    let t11264 = t11262 * t551 * t553;
    let t11268 = t3380 * t547;
    let t11270 = -0.47896936041018436376e-1_f64 * t8471 - 0.12602162889256447134e0_f64 * t8474 + t8477 + 0.12602162889256447134e0_f64 * t8478 - t5977 - 0.23948468020509218188e-1_f64 * t5969 - 0.31505407223141117834e-1_f64 * t5982 - t5986 + t5988 - 0.63010814446282235668e-1_f64 * t5990 - t5993 + 0.63010814446282235668e-1_f64 * t5996 + t5999 - t8489 - 0.63010814446282235668e-1_f64 * t8490 - 0.19753890328909480882e-2_f64 * t11264 - 0.31505407223141117834e-1_f64 * t11162 * t164 - 0.31505407223141117834e-1_f64 * t11268;
    t11270
}
