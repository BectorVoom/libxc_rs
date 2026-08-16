//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1039/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1039(t12309: f64, t1380: f64, t27370: f64, t27333: f64, t27335: f64, t27337: f64, t27339: f64, t27342: f64, t27346: f64, t27349: f64, t27353: f64, t27359: f64, t27362: f64, t27366: f64, t27369: f64, t7908: f64) -> (f64, f64, f64) {
    let t27371 = t12309 * t1380;
    let t27372 = t27370 * t27371;
    let t27375 = 0.49745833333333333332e-2_f64 * t27333 - 0.33163888888888888888e-2_f64 * t27335 + 0.22109259259259259258e-2_f64 * t27337 - 0.185671721767578125e-4_f64 * t27339 * t27342 + 0.46336805555555555556e-3_f64 * t27346 + 0.46336805555555555556e-3_f64 * t27349 + 0.46336805555555555556e-3_f64 * t7908 * t27353 + 0.46336805555555555556e-3_f64 * t7908 * t27359 + 0.22109259259259259258e-2_f64 * t27362 + 0.33163888888888888888e-2_f64 * t27366 - 0.18550940104166666667e-3_f64 * t27369 * t27372;
    (t27371, t27372, t27375)
}
