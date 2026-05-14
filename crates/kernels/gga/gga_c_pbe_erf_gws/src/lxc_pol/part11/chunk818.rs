//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 818/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk818<F: Float>(t16569: F, t5560: F, t24: F, t247: F, t5426: F, t712: F, t248: F, t256: F, t7236: F, t7271: F, t265: F, t266: F, t837: F, t245: F, t5420: F, t5427: F, t723: F) -> (F, F, F, F, F, F) {
    let t18245 = 0.80823369382716049381e-2 * t16569 * t5560;
    let t18261 = 0.24311111111111111111e0 * t712 * t24 * t247 * t5426;
    let t18267 = t248 * (-0.33530864197530864197e0 * t7271 + 0.18360493827160493828e1 * t7236) * t256 / 3.0;
    let t18280 = 56.0 / 1215.0 * t265 * t266 * t837;
    let t18309 = 0.2e-20 * t712 * t245 * t5420;
    let t18311 = 8.0 / 9.0 * t5427 * t723;
    (t18245, t18261, t18267, t18280, t18309, t18311)
}
