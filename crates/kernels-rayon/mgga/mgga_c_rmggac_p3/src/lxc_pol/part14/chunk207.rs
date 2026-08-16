//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 207/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk207(t31: f64, t830: f64, t309: f64, t313: f64, t804: f64, t811: f64, t817: f64, t822: f64, t826: f64, t87: f64, t91: f64, t98: f64) -> (f64, f64, f64) {
    let t831 = t31 * t830;
    let t832 = 22.0_f64 / 9.0_f64 * t831;
    let t833 = 80.0_f64 / 9.0_f64 * t804 * t91 - 100.0_f64 / 9.0_f64 * t309 * t313 + 20.0_f64 / 9.0_f64 * t87 * t811 + 10.0_f64 / 3.0_f64 * t87 * t817 + 20.0_f64 / 9.0_f64 * t98 * t822 + 10.0_f64 / 3.0_f64 * t98 * t826 - t832;
    (t831, t832, t833)
}
