//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 643/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk643(t1017: f64, t2178: f64, t1882: f64, t4807: f64, t4730: f64, t4829: f64, t8392: f64, t1045: f64, t2097: f64, t4805: f64, t604: f64, t16679: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17030 = t2178 * t1017;
    let t17060 = t1882 * t4807;
    let t17091 = t1882 * t4730;
    let t17104 = t8392 * t4829;
    let t17164 = t2097 * t1045;
    let t17198 = t604 * t4805;
    let t17214 = 2.0_f64 / 9.0_f64 * t16679;
    (t17030, t17060, t17091, t17104, t17164, t17198, t17214)
}
