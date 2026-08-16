//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1049/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1049(t300: f64, t6212: f64, t6185: f64, t1642: f64, t4719: f64, t2986: f64, t6189: f64, t973: f64, t981: f64, t6205: f64, t964: f64, t3011: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6213 = t300 * t6212;
    let t6215 = 0.19751673498613801407e-1_f64 * t300 * t6185;
    let t6217 = 0.11696447245269292414e1_f64 * t4719 * t1642;
    let t6219 = t2986 * t6189 * t973;
    let t6221 = 0.11696447245269292414e1_f64 * t981 * t6219;
    let t6223 = t964 * t6205 * t973;
    let t6225 = 0.5848223622634646207e0_f64 * t981 * t6223;
    let t6226 = t3011 * t6189;
    (t6213, t6215, t6217, t6219, t6221, t6223, t6225, t6226)
}
