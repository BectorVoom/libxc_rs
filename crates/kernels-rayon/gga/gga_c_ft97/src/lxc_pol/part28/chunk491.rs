//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 491/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk491(t1349: f64, t1362: f64, t149: f64, t7309: f64, t7315: f64, t7342: f64, t7346: f64, t7396: f64, t7401: f64, t7408: f64, t7412: f64, t7414: f64) -> f64 {
    let t7419 = t7309 * t1362 / 6.0_f64 - t1349 * t7315 / 3.0_f64 + t1349 * t7342 / 6.0_f64 + t1349 * t7346 / 3.0_f64 - t149 * t7412 + 2.0_f64 * t7414 - 4.0_f64 * t7396 + 4.0_f64 * t7401 - 2.0_f64 * t7408;
    t7419
}
