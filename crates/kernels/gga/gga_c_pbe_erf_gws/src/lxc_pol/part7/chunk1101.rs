//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1101/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1101<F: Float>(t2352: F, t2416: F, t353: F, t859: F, t938: F, t19672: F, t19677: F, t19679: F, t19683: F, t19691: F, t19696: F, t19701: F, t19704: F, t2359: F, t2362: F, t2387: F, t2388: F, t4409: F, t6151: F, t6784: F, t6789: F, t6793: F, t6797: F, t827: F) -> F {
    let t19710 = t859 * t353 * t2416 * t2352 * t938;
    let t19713 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t2388 * t6151 - t827 * t19672 / F::cast_from(4.0_f64) - t2388 * t6789 / F::cast_from(8.0_f64) + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t19677 - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t19679 - t2359 * t19683 / F::cast_from(24.0_f64) - t2388 * t6784 / F::cast_from(8.0_f64) - t2387 * t4409 * t2362 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t19691 - t827 * t19696 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t6793 * t19701 + t19704 * t6797 / F::cast_from(4.0_f64) + t6793 * t19710 / F::cast_from(4.0_f64);
    t19713
}
