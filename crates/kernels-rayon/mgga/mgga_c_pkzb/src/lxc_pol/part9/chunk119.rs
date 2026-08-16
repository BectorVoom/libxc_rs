//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 119/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk119(t24: f64, t18: f64, t326: f64, t91: f64, dens_threshold: f64, rho1: f64, tau1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t90 = t24 <= zeta_threshold;
    let t327 = 1.0_f64 / t18;
    let t328 = t326 * t327;
    let t329 = tau1 * tau1;
    let t330 = 1.0_f64 / t329;
    let t332 = rho1 <= dens_threshold || t90;
    let t333 = 1.0_f64 / t91;
    (t328, t330, t333)
}
