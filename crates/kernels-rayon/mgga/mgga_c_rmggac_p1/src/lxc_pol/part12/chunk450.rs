//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 450/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk450(t1302: f64, t1338: f64, t1343: f64, t270: f64, t271: f64, t4710: f64, t4712: f64, t4720: f64, t4781: f64, t4787: f64, t4789: f64, t71: f64) -> f64 {
    let t4792 = 0.1714584e0_f64 * t4710 - 0.1714584e0_f64 * t4712 * t1302 + 0.285764e-1_f64 * t4720 + 0.285764e-1_f64 * t4781 * t271 - 0.857292e-1_f64 * t1338 * t1343 * t270 + 0.571528e-1_f64 * t4787 * t4789;
    let t4793 = t4792 * t71;
    t4793
}
