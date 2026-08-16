//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 445/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk445(t1029: f64, t1030: f64, t1697: f64, t1717: f64, t1728: f64, t1745: f64, t278: f64, t305: f64, t339: f64) -> f64 {
    let t1747 = -t1029 - 0.23426533963880895498e-2_f64 * t1030 * t1717 - 0.46853067927761790996e-2_f64 * t305 * t1728 - t1697 * t339 - t278 * t1745;
    t1747
}
