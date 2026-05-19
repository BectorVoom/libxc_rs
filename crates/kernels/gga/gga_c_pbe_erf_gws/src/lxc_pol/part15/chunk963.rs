//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 963/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk963<F: Float>(t164: F, t3013: F, t8048: F, t2519: F, t547: F, t2523: F, t1964: F, t992: F, t5969: F, t5973: F, t5977: F, t5980: F, t5982: F, t5986: F, t5988: F, t5990: F, t5993: F, t5994: F, t5996: F, t5999: F, t8053: F) -> F {
    let t8474 = t3013 * t164;
    let t8477 = F::cast_from(0.63010814446282235668e-1_f64) * t8048 * t164;
    let t8478 = t2519 * t547;
    let t8489 = F::cast_from(0.63010814446282235668e-1_f64) * t2523 * t547;
    let t8490 = t992 * t1964;
    let t8493 = -F::cast_from(0.63010814446282235668e-1_f64) * t8474 + t8477 + F::cast_from(0.63010814446282235668e-1_f64) * t8478 - t5977 - F::cast_from(0.47896936041018436376e-1_f64) * t5969 - F::cast_from(0.31505407223141117834e-1_f64) * t5980 - F::cast_from(0.63010814446282235668e-1_f64) * t5982 - t5986 + t5988 - F::cast_from(0.12602162889256447134e0_f64) * t5990 - t5993 + F::cast_from(0.31505407223141117834e-1_f64) * t5994 + F::cast_from(0.12602162889256447134e0_f64) * t5996 + t5999 - F::cast_from(0.31505407223141117834e-1_f64) * t8053 * t164 - t8489 - F::cast_from(0.31505407223141117834e-1_f64) * t8490 + F::cast_from(0.89806755076909568204e-2_f64) * t5973;
    t8493
}
