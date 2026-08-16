//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 141/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk141(t24: f64, t422: f64, t423: f64, t330: f64, t30: f64, t42: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t426 = piecewise3(t332, 0.0_f64, t422 * t423 / 2.0_f64);
    let t427 = t330 * t426;
    let t430 = t30 * t42;
    (t427, t430)
}
