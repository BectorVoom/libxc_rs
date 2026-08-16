//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1115/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1115(t236: f64, t29219: f64, t233: f64, t235: f64, t6883: f64, t2169: f64, t441: f64, t6293: f64, t1657: f64, t1876: f64, t2209: f64, t6294: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29220 = t236 * t29219;
    let t29221 = t233 * t29220;
    let t29222 = t29221 / 16.0_f64;
    let t29223 = t235 * t6883;
    let t29224 = t2169 * t29223;
    let t29225 = t29224 / 16.0_f64;
    let t29226 = t6293 * t441;
    let t29227 = t2169 * t29226;
    let t29228 = t29227 / 16.0_f64;
    let t29229 = t1657 * t1876;
    let t29230 = t2169 * t29229;
    let t29231 = t29230 / 8.0_f64;
    let t29232 = t6294 * t2209;
    (t29220, t29222, t29223, t29225, t29226, t29228, t29229, t29231, t29232)
}
