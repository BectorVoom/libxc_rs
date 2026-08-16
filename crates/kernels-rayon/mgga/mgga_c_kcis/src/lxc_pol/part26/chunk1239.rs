//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1239/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1239(t1467: f64, t97800: f64, t1928: f64, t4254: f64, t1532: f64, t572: f64, t4188: f64, t8182: f64, t28450: f64, t4142: f64, t27376: f64, t28392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97801 = t1467 * t97800;
    let t97804 = t4254 * t1928;
    let t97821 = t1532 * t572;
    let t97991 = t8182 * t4188;
    let t97997 = t4142 * t28450;
    let t98016 = t28392 * t27376;
    (t97801, t97804, t97821, t97991, t97997, t98016)
}
