//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 798/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk798(t109: f64, t8319: f64, t89: f64, t510: f64) -> (f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t8320 = t89 * t8319;
    let t8322 = 2.0_f64 * t8320 * t510;
    let t8326 = piecewise3(t110, 0.0_f64, 0.0_f64);
    (t8320, t8322, t8326)
}
