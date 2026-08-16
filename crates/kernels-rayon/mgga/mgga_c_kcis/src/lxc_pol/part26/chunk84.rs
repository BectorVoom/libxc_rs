//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 84/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk84(t107: f64, t6: f64, t109: f64, t251: f64, t118: f64, t7: f64, t41: f64, t66: f64) -> (f64, f64, f64, f64) {
    let t323 = t107 * t6;
    let t324 = t109 * t251;
    let t333 = t118 * t7;
    let t334 = t66 * t41;
    (t323, t324, t333, t334)
}
