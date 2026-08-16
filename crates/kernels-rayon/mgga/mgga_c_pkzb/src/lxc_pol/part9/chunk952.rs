//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 952/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk952(t7335: f64, t5522: f64, t7332: f64, t7352: f64, t7361: f64, t7363: f64, t7366: f64, t7368: f64, t7371: f64, t7373: f64, t7376: f64, t7379: f64) -> f64 {
    let t7420 = 0.59793333333333333334e0_f64 * t7335;
    let t7431 = 0.27385555555555555555e0_f64 * t7332 - t7420 + 0.8969e0_f64 * t7352 + 0.3071625e0_f64 * t7361 + 0.1898925e1_f64 * t7363 - 0.1898925e1_f64 * t7366 - 0.9494625e0_f64 * t7368 + 0.3071625e0_f64 * t7371 + 0.15358125e0_f64 * t7373 + 0.142419375e1_f64 * t7376 - 0.76790625e-1_f64 * t7379 + 0.79724444444444444446e0_f64 * t5522;
    t7431
}
