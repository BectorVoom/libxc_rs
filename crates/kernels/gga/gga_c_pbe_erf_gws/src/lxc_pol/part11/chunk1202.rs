//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1202/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1202<F: Float>(t10186: F, t11296: F, t12406: F, t12899: F, t12962: F, t142: F, t143: F, t1503: F, t169: F, t19169: F, t19174: F, t19182: F, t19203: F, t19458: F, t19466: F, t2031: F, t26477: F, t26480: F, t296: F, t299: F, t301: F, t33446: F, t34300: F, t34326: F, t3638: F, t3671: F, t42905: F, t43168: F, t48520: F, t48741: F, t48908: F, t526: F, t5651: F, t8497: F, t967: F, t987: F, t988: F) -> F {
    let t48932 = -F::cast_from(0.36991419282863461287e1_f64) * t26477 - F::cast_from(0.3486808982146430324e-2_f64) * t26480 - t988 * t2031 * t142 * t12962 + F::cast_from(18.0_f64) * t11296 * t10186 + t48908 * t296 - t19169 - t19174 - F::cast_from(0.47896936041018436376e-1_f64) * t43168 + t19182 + F::cast_from(0.20267214298646782767e-1_f64) * t169 * t299 * t48520 * t301 - F::cast_from(0.10931146159029059066e-3_f64) * t34300 + F::cast_from(18.0_f64) * t1503 * t143 * t48741 + F::cast_from(0.23948468020509218188e0_f64) * t34326 + F::cast_from(6.0_f64) * t988 * t33446 * t12406 + F::cast_from(24.0_f64) * t12899 * t987 * t526 - t19203 + F::cast_from(36.0_f64) * t42905 * t3638 - t19458 + t19466 - F::cast_from(12.0_f64) * t8497 * t5651 * t3671 * t967;
    t48932
}
