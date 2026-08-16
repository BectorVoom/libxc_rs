//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 392/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk392(t20: f64, t2314: f64, t92: f64, t2: f64, t647: f64, t725: f64, t2318: f64, t2321: f64, t2323: f64, t15: f64, t2317: f64, t2320: f64, t650: f64, t720: f64) -> (f64, f64, f64, f64) {
    let t2444 = t2314 * t92 * t20;
    let t2448 = t647 * t725 * t2;
    let t2456 = -0.44044444444444444445e-2_f64 * t2318 + 0.88088888888888888889e-2_f64 * t2321 + 0.55033333333333333333e-2_f64 * t2323;
    let t2459 = -t2444 * t2317 / 18.0_f64 - t2448 * t650 / 6.0_f64 + t720 * t2320 / 9.0_f64 + t15 * t2456 / 2.0_f64;
    (t2444, t2448, t2456, t2459)
}
