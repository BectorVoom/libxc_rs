//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 653/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk653(t25596: f64, t83: f64, t3271: f64, t452: f64, t5710: f64, t1901: f64, t23183: f64, t23199: f64, t23227: f64, t23229: f64, t26364: f64, t26368: f64, t26375: f64, t26379: f64, t26383: f64, t26387: f64, t26392: f64, t26395: f64, t446: f64) -> f64 {
    let t26398 = t83 * t25596;
    let t26402 = t452 * t5710 * t3271;
    let t26407 = t1901 * t26364 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t26368 - 2.0_f64 * t1901 * t26375 - 2.0_f64 / 3.0_f64 * t1901 * t26379 - 2.0_f64 / 3.0_f64 * t1901 * t26383 - t23183 - t23199 / 9.0_f64 - t446 * t26387 / 3.0_f64 + t446 * t26392 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t26395 + 2.0_f64 / 3.0_f64 * t446 * t26398 + t446 * t26402 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t23227 - t23229 / 9.0_f64;
    t26407
}
