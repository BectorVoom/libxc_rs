//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1039/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1039<F: Float>(t12309: F, t1380: F, t27370: F, t27333: F, t27335: F, t27337: F, t27339: F, t27342: F, t27346: F, t27349: F, t27353: F, t27359: F, t27362: F, t27366: F, t27369: F, t7908: F) -> (F, F, F) {
    let t27371 = t12309 * t1380;
    let t27372 = t27370 * t27371;
    let t27375 = F::cast_from(0.49745833333333333332e-2_f64) * t27333 - F::cast_from(0.33163888888888888888e-2_f64) * t27335 + F::cast_from(0.22109259259259259258e-2_f64) * t27337 - F::cast_from(0.185671721767578125e-4_f64) * t27339 * t27342 + F::cast_from(0.46336805555555555556e-3_f64) * t27346 + F::cast_from(0.46336805555555555556e-3_f64) * t27349 + F::cast_from(0.46336805555555555556e-3_f64) * t7908 * t27353 + F::cast_from(0.46336805555555555556e-3_f64) * t7908 * t27359 + F::cast_from(0.22109259259259259258e-2_f64) * t27362 + F::cast_from(0.33163888888888888888e-2_f64) * t27366 - F::cast_from(0.18550940104166666667e-3_f64) * t27369 * t27372;
    (t27371, t27372, t27375)
}
