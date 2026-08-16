//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 714/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk714(t1212: f64, t6386: f64, t840: f64, t871: f64, t681: f64, t7093: f64, t89: f64, t25298: f64, t25312: f64, t25315: f64, t25317: f64, t25366: f64, t29378: f64, t29383: f64, t29385: f64, t29387: f64, t29389: f64, t29392: f64, t29396: f64, t446: f64) -> (f64, f64) {
    let t29399 = t6386 * t1212;
    let t29401 = t840 * t871 * t29399;
    let t29405 = t89 * t681 * t7093;
    let t29407 = t25298 / 9.0_f64 + t25312 / 27.0_f64 - t446 * t29378 / 3.0_f64 - t25315 / 9.0_f64 + t25317 / 9.0_f64 - t29383 / 9.0_f64 + t29385 / 9.0_f64 + t29387 / 9.0_f64 - t446 * t29389 / 3.0_f64 + t29392 / 9.0_f64 + t25366 / 9.0_f64 + t446 * t29396 / 3.0_f64 + t446 * t29401 / 3.0_f64 - t29405 / 9.0_f64;
    (t29399, t29407)
}
