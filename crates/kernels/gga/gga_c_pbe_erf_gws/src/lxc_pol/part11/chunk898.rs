//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 898/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk898<F: Float>(t5385: F, t720: F, t1365: F, t252: F, t254: F, t16569: F, t5560: F, t24: F, t247: F, t5426: F, t712: F, t248: F, t256: F, t7236: F, t7271: F) -> (F, F, F, F, F) {
    let t18240 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t720 * t5385;
    let t18243 = F::cast_from(56.0_f64) / F::cast_from(243.0_f64) * t252 * t254 * t1365;
    let t18245 = F::cast_from(0.80823369382716049381e-2_f64) * t16569 * t5560;
    let t18261 = F::cast_from(0.24311111111111111111e0_f64) * t712 * t24 * t247 * t5426;
    let t18267 = t248 * (-F::cast_from(0.33530864197530864197e0_f64) * t7271 + F::cast_from(0.18360493827160493828e1_f64) * t7236) * t256 / F::cast_from(3.0_f64);
    (t18240, t18243, t18245, t18261, t18267)
}
