//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1157/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1157(t13605: f64, t1526: f64, t21123: f64, t21125: f64, t21181: f64, t21399: f64, t21442: f64, t21457: f64, t231: f64, t2320: f64, t342: f64, t343: f64, t3806: f64, t42293: f64, t42307: f64, t69073: f64, t69137: f64, t81955: f64, t81958: f64) -> f64 {
    let t89656 = -t342 * t343 * t231 * t21399 / 4.0_f64 + t21123 - t42293 - t1526 * t3806 * t21442 / 3.0_f64 - 7.0_f64 / 27.0_f64 * t1526 * t13605 * t42307 * t21181 - t1526 * t2320 * t21457 / 4.0_f64 - t81955 / 9.0_f64 - t81958 / 6.0_f64 + t69073 / 6.0_f64 + t69137 / 18.0_f64 + 2.0_f64 * t21125;
    t89656
}
