//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1206/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1206<F: Float>(t21430: F, t2281: F, t20695: F, t274: F, t20432: F, t6328: F, t8782: F, t19561: F, t20527: F, t20571: F, t20708: F, t21399: F, t21400: F, t21405: F, t21412: F, t21414: F, t21424: F, t21429: F, t2255: F, t2277: F, t2345: F, t254: F, t3247: F, t3257: F, t6275: F, t6282: F, t820: F, t906: F, t9568: F) -> (F, F) {
    let t21431 = t21430 * t2281;
    let t21438 = t274 * t20695;
    let t21445 = t8782 * t20432 * t6328 / F::new(16.0);
    let t21446 = -F::new(5.0) / F::new(16.0) * t21399 * t254 * t21400 * t906 + t6275 * t20527 * t21405 / F::new(8.0) + t21412 - t21414 - F::new(7.0) / F::new(384.0) * t2277 * t3257 * t20571 * t9568 - t21424 + t21429 - F::new(119.0) / F::new(1152.0) * t21431 - F::new(3.0) / F::new(64.0) * t3247 * t2345 * t6282 * t20708 - t2277 * t2255 * t820 * t19561 * t21438 / F::new(256.0) + t21445;
    (t21445, t21446)
}
