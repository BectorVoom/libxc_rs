//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1237/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1237<F: Float>(t2074: F, t2395: F, t2370: F, t830: F, t3074: F, t4473: F, t6772: F, t6130: F, t840: F, t19505: F, t19553: F, t19602: F, t21819: F, t21823: F, t21826: F, t21830: F, t2306: F, t2373: F, t2376: F, t2401: F, t2409: F, t2410: F, t3066: F, t3067: F, t3079: F, t3207: F, t328: F, t338: F, t353: F, t376: F, t4427: F, t6104: F, t6385: F, t6723: F, t827: F, t844: F, t9241: F, t9283: F, t938: F) -> F {
    let t21832 = t2395 * t2074;
    let t21834 = t2370 * t830 * t21832;
    let t21845 = t3074 * t4473 * t6772;
    let t21847 = t840 * t6130;
    let t21867 = -t844 * t338 * t353 * t376 * t19553 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t21819 + t21823 + F::cast_from(7.0_f64) / F::cast_from(3.0_f64) * t21826 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t21830 - t827 * t21834 / F::cast_from(4.0_f64) - t4427 * t2373 / F::cast_from(6.0_f64) + t3074 * t2306 * t6104 * t328 * t3079 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t21845 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t21847 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2401 * t338 * t353 * t376 * t19505 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3207 * t9283 * t19602 * t2410 + t3066 * t2409 * t3067 * t6723 * t938 / F::cast_from(12.0_f64) + t9241 * t2409 * t2376 * t6385 * t938;
    t21867
}
