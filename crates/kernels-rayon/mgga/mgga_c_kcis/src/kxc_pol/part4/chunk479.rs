//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 479/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk479(t1403: f64, t1404: f64, t1924: f64, t1951: f64, t1962: f64, t1979: f64, t486: f64, t510: f64, t538: f64) -> f64 {
    let t1981 = -t1403 - 0.23426533963880895498e-2_f64 * t1404 * t1951 - 0.46853067927761790996e-2_f64 * t510 * t1962 - t1924 * t538 - t486 * t1979;
    t1981
}
