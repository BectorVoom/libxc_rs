//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 972/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk972<F: Float>(t2359: F, t2373: F, t2388: F, t2392: F, t2498: F, t2503: F, t3040: F, t3047: F, t3077: F, t3207: F, t4415: F, t6793: F, t827: F, t833: F, t8584: F, t8592: F, t8598: F, t8602: F, t8606: F, t8611: F, t8616: F, t8622: F, t8624: F) -> F {
    let t8628 = -t2388 * t3047 / F::cast_from(96.0_f64) - t2392 * t3047 / F::cast_from(96.0_f64) - t827 * t8584 / F::cast_from(48.0_f64) - t3040 * t2373 / F::cast_from(24.0_f64) - t827 * t8592 / F::cast_from(48.0_f64) - t2498 * t2373 / F::cast_from(24.0_f64) + t8598 + t6793 * t8602 / F::cast_from(8.0_f64) + t3077 * t8606 / F::cast_from(48.0_f64) - t2359 * t8611 / F::cast_from(96.0_f64) + t2388 * t2503 / F::cast_from(96.0_f64) + t8616 * t833 / F::cast_from(96.0_f64) + t2392 * t2503 / F::cast_from(96.0_f64) + t8622 + t3207 * t8624 / F::cast_from(8.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4415;
    t8628
}
