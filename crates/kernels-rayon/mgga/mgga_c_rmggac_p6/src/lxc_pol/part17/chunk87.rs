//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 87/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk87(t20: f64, t43: f64, t40: f64, t41: f64, t21: f64, t22: f64) -> (f64, f64, f64, f64) {
    let t255 = t20 * t43;
    let t259 = 1.0_f64 / t41 / t40;
    let t260 = t21 * t259;
    let t261 = t260 * t22;
    (t255, t259, t260, t261)
}
