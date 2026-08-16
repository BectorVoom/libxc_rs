//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 785/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk785(t1980: f64, t2672: f64, t1392: f64, t2581: f64, t1391: f64, t2571: f64, t2013: f64, t2680: f64, t2012: f64, t2683: f64) -> (f64, f64, f64, f64, f64) {
    let t7403 = t1980 * t2672;
    let t7406 = t1392 * t2581;
    let t7407 = t1391 * t7406;
    let t7410 = t1392 * t2571;
    let t7411 = t1391 * t7410;
    let t7414 = t2013 * t2680;
    let t7416 = t2012 * t2683;
    (t7403, t7407, t7411, t7414, t7416)
}
