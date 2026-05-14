//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 982/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk982<F: Float>(t19693: F, t19694: F, t830: F, t2182: F, t353: F, t8599: F, t898: F, t938: F, t2387: F, t6792: F, t2352: F, t2416: F, t859: F, t19672: F, t19677: F, t19679: F, t19683: F, t19691: F, t2359: F, t2362: F, t2388: F, t4409: F, t6151: F, t6784: F, t6789: F, t6793: F, t6797: F, t827: F) -> (F,) {
    let t19696 = t19693 * t830 * t19694;
    let t19701 = t8599 * t353 * t898 * t2182 * t938;
    let t19704 = t2387 * t6792;
    let t19710 = t859 * t353 * t2416 * t2352 * t938;
    let t19713 = 3.0 / 8.0 * t2388 * t6151 - t827 * t19672 / 4.0 - t2388 * t6789 / 8.0 + 7.0 / 12.0 * t19677 - 35.0 / 36.0 * t19679 - t2359 * t19683 / 24.0 - t2388 * t6784 / 8.0 - t2387 * t4409 * t2362 / 16.0 + 7.0 / 24.0 * t19691 - t827 * t19696 - 3.0 / 4.0 * t6793 * t19701 + t19704 * t6797 / 4.0 + t6793 * t19710 / 4.0;
    (t19713,)
}
