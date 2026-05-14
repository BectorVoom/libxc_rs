//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 887/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk887<F: Float>(t2306: F, t8703: F, t3074: F, t2501: F, t810: F, t2370: F, t830: F, t1105: F, t898: F, t938: F, t353: F, t4386: F, t1115: F, t2384: F, t3047: F, t3052: F, t3079: F, t335: F, t4385: F, t4475: F, t4477: F, t6135: F, t6151: F, t6789: F, t6793: F, t827: F, t8671: F, t8677: F, t8685: F, t8690: F, t8695: F, t8700: F) -> (F, F, F, F) {
    let t8704 = t2306 * t8703;
    let t8705 = t3074 * t8704;
    let t8708 = t2501 * t810;
    let t8710 = t2370 * t830 * t8708;
    let t8713 = t898 * t1105;
    let t8714 = t8713 * t938;
    let t8715 = t353 * t8714;
    let t8716 = t4386 * t8715;
    let t8721 = -t8671 - t1115 * t6135 / 24.0 - t1115 * t6789 / 48.0 + t8677 + t1115 * t6151 / 16.0 - t2384 * t3047 / 96.0 - t2384 * t3052 / 48.0 - t335 * t8685 / 48.0 + t4385 * t8690 / 96.0 + t6793 * t8695 / 24.0 + t4385 * t8700 / 48.0 + t8705 * t3079 / 48.0 - t827 * t8710 / 24.0 + t6793 * t8716 / 24.0 - 7.0 / 288.0 * t4475 - 7.0 / 288.0 * t4477;
    (t8708, t8713, t8716, t8721)
}
