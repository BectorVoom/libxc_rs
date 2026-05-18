//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 978/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk978<F: Float>(t1105: F, t898: F, t938: F, t353: F, t4386: F, t1115: F, t2384: F, t3047: F, t3052: F, t3079: F, t335: F, t4385: F, t4475: F, t4477: F, t6135: F, t6151: F, t6789: F, t6793: F, t827: F, t8671: F, t8677: F, t8685: F, t8690: F, t8695: F, t8700: F, t8705: F, t8710: F) -> (F, F, F) {
    let t8713 = t898 * t1105;
    let t8714 = t8713 * t938;
    let t8715 = t353 * t8714;
    let t8716 = t4386 * t8715;
    let t8721 = -t8671 - t1115 * t6135 / F::new(24.0) - t1115 * t6789 / F::new(48.0) + t8677 + t1115 * t6151 / F::new(16.0) - t2384 * t3047 / F::new(96.0) - t2384 * t3052 / F::new(48.0) - t335 * t8685 / F::new(48.0) + t4385 * t8690 / F::new(96.0) + t6793 * t8695 / F::new(24.0) + t4385 * t8700 / F::new(48.0) + t8705 * t3079 / F::new(48.0) - t827 * t8710 / F::new(24.0) + t6793 * t8716 / F::new(24.0) - F::new(7.0) / F::new(288.0) * t4475 - F::new(7.0) / F::new(288.0) * t4477;
    (t8713, t8716, t8721)
}
