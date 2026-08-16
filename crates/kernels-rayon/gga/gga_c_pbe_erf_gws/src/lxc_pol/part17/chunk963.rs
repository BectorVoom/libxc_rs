//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 963/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk963(t164: f64, t3013: f64, t8048: f64, t2519: f64, t547: f64, t2523: f64, t1964: f64, t992: f64, t5969: f64, t5973: f64, t5977: f64, t5980: f64, t5982: f64, t5986: f64, t5988: f64, t5990: f64, t5993: f64, t5994: f64, t5996: f64, t5999: f64, t8053: f64) -> f64 {
    let t8474 = t3013 * t164;
    let t8477 = 0.63010814446282235668e-1_f64 * t8048 * t164;
    let t8478 = t2519 * t547;
    let t8489 = 0.63010814446282235668e-1_f64 * t2523 * t547;
    let t8490 = t992 * t1964;
    let t8493 = -0.63010814446282235668e-1_f64 * t8474 + t8477 + 0.63010814446282235668e-1_f64 * t8478 - t5977 - 0.47896936041018436376e-1_f64 * t5969 - 0.31505407223141117834e-1_f64 * t5980 - 0.63010814446282235668e-1_f64 * t5982 - t5986 + t5988 - 0.12602162889256447134e0_f64 * t5990 - t5993 + 0.31505407223141117834e-1_f64 * t5994 + 0.12602162889256447134e0_f64 * t5996 + t5999 - 0.31505407223141117834e-1_f64 * t8053 * t164 - t8489 - 0.31505407223141117834e-1_f64 * t8490 + 0.89806755076909568204e-2_f64 * t5973;
    t8493
}
