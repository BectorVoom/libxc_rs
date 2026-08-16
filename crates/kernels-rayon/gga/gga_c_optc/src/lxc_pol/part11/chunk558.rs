//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 558/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk558(t43: f64, t1884: f64, t4561: f64, t4565: f64, t47: f64, t1239: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t4569 = piecewise3(t44, 0.0_f64, 4.0_f64 / 9.0_f64 * t1884 * t4561 + 4.0_f64 / 3.0_f64 * t47 * t4565);
    let t4570 = t1239 * t1239;
    (t4569, t4570)
}
