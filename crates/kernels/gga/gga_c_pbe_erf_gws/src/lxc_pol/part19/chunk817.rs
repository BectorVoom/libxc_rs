//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 817/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk817<F: Float>(t2370: F, t830: F, t9888: F, t1115: F, t2397: F, t2408: F, t3207: F, t335: F, t3917: F, t4425: F, t4430: F, t4443: F, t827: F, t8622: F, t8641: F, t8643: F, t8646: F, t8664: F, t8666: F, t8710: F, t9865: F, t9869: F, t9873: F, t9879: F, t9885: F) -> (F,) {
    let t9890 = t2370 * t830 * t9888;
    let t9893 = t3917 * t2397 / 96.0 + t335 * t9865 / 48.0 + t8622 + t2408 * t9869 / 24.0 - t3207 * t9873 / 8.0 + 35.0 / 432.0 * t4425 - 35.0 / 432.0 * t4430 - 35.0 / 216.0 * t4443 - 7.0 / 144.0 * t9879 + t8641 + t8643 + t8646 + t8664 - t1115 * t8710 / 24.0 - t827 * t9885 / 48.0 - t827 * t9890 / 48.0 - t8666;
    (t9893,)
}
