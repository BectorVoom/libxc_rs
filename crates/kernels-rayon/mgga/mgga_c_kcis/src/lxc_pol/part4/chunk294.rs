//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 294/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk294(t1029: f64, t1030: f64, t1031: f64, t1046: f64, t1083: f64, t278: f64, t305: f64, t339: f64, t975: f64) -> f64 {
    let t1085 = -t1029 - 0.23426533963880895498e-2_f64 * t1030 * t1031 - 0.46853067927761790996e-2_f64 * t305 * t1046 - t975 * t339 - t278 * t1083;
    t1085
}
