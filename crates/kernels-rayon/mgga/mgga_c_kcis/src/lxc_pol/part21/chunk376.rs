//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 376/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk376(t2179: f64, t342: f64, t303: f64, t2173: f64, t2175: f64, t393: f64, t374: f64, t377: f64, t1021: f64, t389: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2180 = t342 * t2179;
    let t2181 = t303 * t2180;
    let t2183 = -0.69505208333333333333e-3_f64 * t2173 * t2175 + 0.24872916666666666666e-2_f64 * t2181;
    let t2184 = t2183 * t393;
    let t2185 = t374 * t377;
    let t2187 = t1021 * t389;
    let t2189 = t2185 / 16.0_f64 - t2187 / 128.0_f64;
    (t2180, t2181, t2183, t2184, t2185, t2187, t2189)
}
