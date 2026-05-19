//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 999/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk999<F: Float>(t1918: F, t5451: F, t24: F, t247: F, t5426: F, t712: F, t248: F, t256: F, t7236: F, t7271: F, t5448: F, t723: F) -> (F, F, F, F) {
    let t18256 = t5451 * t1918;
    let t18261 = F::cast_from(0.24311111111111111111e0_f64) * t712 * t24 * t247 * t5426;
    let t18267 = t248 * (-F::cast_from(0.33530864197530864197e0_f64) * t7271 + F::cast_from(0.18360493827160493828e1_f64) * t7236) * t256 / F::new(3.0);
    let t18268 = t5448 * t723;
    (t18256, t18261, t18267, t18268)
}
