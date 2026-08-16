//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 314/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk314(t1167: f64, t1173: f64, t1177: f64, t1181: f64, t1185: f64, t1190: f64, t1197: f64, t1201: f64) -> f64 {
    let t1291 = 0.9375e-1_f64 * t1167 - 0.9375e-1_f64 * t1173 - 0.25e0_f64 * t1177 + 0.625e-1_f64 * t1181 - 0.101171875e-1_f64 * t1185 + 0.101171875e-1_f64 * t1190 + 0.53958333333333333333e-1_f64 * t1197 - 0.13489583333333333333e-1_f64 * t1201;
    t1291
}
