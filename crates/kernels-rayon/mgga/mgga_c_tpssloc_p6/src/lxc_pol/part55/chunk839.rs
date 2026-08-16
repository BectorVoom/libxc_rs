//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 839/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk839(t8444: f64, t8446: f64, t8684: f64, t8860: f64, t113: f64, t2114: f64, t2165: f64, t510: f64, t574: f64, t8322: f64, t8329: f64, t8491: f64, t8495: f64, t8669: f64, t8676: f64, t8691: f64, t8913: f64) -> (f64, f64) {
    let t8916 = t8860 + 4.0_f64 * t8684 + t8444 + t8446;
    let t8919 = -t113 * t8913 - 2.0_f64 * t2114 * t2165 - t510 * t8860 + t574 * t8916 - t8322 - t8329 + t8491 - t8495 - 4.0_f64 * t8669 - 4.0_f64 * t8676 + 2.0_f64 * t8691;
    (t8916, t8919)
}
