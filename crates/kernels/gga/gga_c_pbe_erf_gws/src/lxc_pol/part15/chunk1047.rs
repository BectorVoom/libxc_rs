//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1047/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1047<F: Float>(t2158: F, t3219: F, t3235: F, t2323: F, t3268: F, t1113: F, t904: F, t6278: F, t2277: F, t3247: F, t6275: F, t6579: F, t8960: F, t8965: F, t8969: F, t8971: F, t8973: F, t8977: F, t9478: F, t9485: F, t9490: F) -> (F, F, F, F) {
    let t9494 = t3235 * t3219 * t2158;
    let t9498 = F::new(7.0) / F::new(576.0) * t2323 * t3268;
    let t9499 = t904 * t1113;
    let t9500 = t9499 * t6278;
    let t9503 = t8960 - t8965 - t2277 * t9478 / F::new(768.0) - t8969 + t2277 * t9485 / F::new(384.0) + F::new(5.0) / F::new(384.0) * t6579 * t9490 + F::new(3.0) / F::new(512.0) * t3247 * t9494 + t8971 + t9498 + t8973 - t8977 + t6275 * t9500 / F::new(96.0);
    (t9494, t9499, t9500, t9503)
}
