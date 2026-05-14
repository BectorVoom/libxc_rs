//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 912/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk912<F: Float>(t331: F, t3379: F, t551: F, t553: F, t3380: F, t547: F, t11162: F, t164: F, t5969: F, t5977: F, t5982: F, t5986: F, t5988: F, t5990: F, t5993: F, t5996: F, t5999: F, t8471: F, t8474: F, t8477: F, t8478: F, t8489: F, t8490: F) -> (F,) {
    let t11262 = t331 * t3379;
    let t11264 = t11262 * t551 * t553;
    let t11268 = t3380 * t547;
    let t11270 = -0.47896936041018436376e-1 * t8471 - 0.12602162889256447134e0 * t8474 + t8477 + 0.12602162889256447134e0 * t8478 - t5977 - 0.23948468020509218188e-1 * t5969 - 0.31505407223141117834e-1 * t5982 - t5986 + t5988 - 0.63010814446282235668e-1 * t5990 - t5993 + 0.63010814446282235668e-1 * t5996 + t5999 - t8489 - 0.63010814446282235668e-1 * t8490 - 0.19753890328909480882e-2 * t11264 - 0.31505407223141117834e-1 * t11162 * t164 - 0.31505407223141117834e-1 * t11268;
    (t11270,)
}
