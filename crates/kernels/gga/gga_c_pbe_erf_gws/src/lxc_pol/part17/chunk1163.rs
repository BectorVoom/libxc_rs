//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1163/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1163<F: Float>(t3959: F, t8756: F, t14576: F, t2376: F, t829: F, t830: F, t13972: F, t14608: F, t1193: F, t2410: F, t3207: F, t36200: F, t36201: F, t4155: F, t50919: F, t50924: F, t51906: F, t54461: F, t54464: F, t54465: F, t54473: F, t54480: F, t54482: F, t827: F, t8629: F, t8759: F, t8793: F, t8804: F, t9283: F) -> (F,) {
    let t54484 = t3959 * t8756;
    let t54486 = t2376 * t14576;
    let t54488 = t829 * t830 * t54486;
    let t54491 = t13972 * t14608;
    let t54492 = 7.0 / 2304.0 * t54491;
    let t54493 = -t3207 * t9283 * t1193 * t8804 / 8.0 - t3207 * t9283 * t1193 * t8759 / 16.0 + t54461 / 3072.0 - t54464 + t54465 / 48.0 + t36200 * t36201 * t4155 * t2410 / 4.0 - t54473 / 384.0 - t8793 * t50919 / 12.0 - t8629 * t50924 / 24.0 + t54480 + t54482 + 7.0 / 288.0 * t51906 + t54484 / 24.0 - t827 * t54488 / 48.0 + t54492;
    (t54493,)
}
