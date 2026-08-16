//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 304/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk304(t1200: f64, t382: f64, t1167: f64, t1173: f64, t1177: f64, t1181: f64, t1185: f64, t1190: f64, t1197: f64) -> (f64, f64) {
    let t1201 = t382 * t1200;
    let t1203 = t1167 / 16.0_f64 - t1173 / 16.0_f64 - t1177 / 6.0_f64 + t1181 / 24.0_f64 - t1185 / 256.0_f64 + t1190 / 256.0_f64 + t1197 / 48.0_f64 - t1201 / 192.0_f64;
    (t1201, t1203)
}
