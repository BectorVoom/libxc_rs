//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 618/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk618<F: Float>(t2494: F, t904: F, t933: F, t1158: F, t2323: F, t1150: F, t2319: F, t2204: F, t2320: F, t2324: F, t2336: F, t3174: F, t3175: F, t3188: F, t3197: F, t929: F) -> (F, F, F, F) {
    let t3268 = t933 * t904 * t2494;
    let t3271 = t2323 * t1158;
    let t3274 = t2319 * t1150;
    let t3277 = t2204 - t3175 - t929 * t3268 / F::new(768.0) + F::new(7.0) / F::new(1152.0) * t3271 - t3174 + t3188 - t3197 - F::new(7.0) / F::new(2304.0) * t2320 - F::new(7.0) / F::new(2304.0) * t3274 + F::new(7.0) / F::new(1152.0) * t2324 + t2336;
    (t3268, t3271, t3274, t3277)
}
