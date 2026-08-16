//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 39/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk39(t101: f64, t99: f64, t31: f64, t36: f64, t87: f64, t91: f64, t98: f64) -> (f64, f64) {
    let t102 = t101 * t99;
    let t107 = 2.0_f64 * t87 * t91 + 2.0_f64 * t98 * t102 - t31 * t36 / 4.0_f64;
    (t102, t107)
}
