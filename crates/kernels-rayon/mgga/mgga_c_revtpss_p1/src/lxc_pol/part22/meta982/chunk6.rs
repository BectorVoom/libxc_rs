//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3329/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3329(t18838: f64, t2411: f64, t4537: f64, t890: f64, t14436: f64, t18256: f64, t1940: f64, t50080: f64, t62297: f64, t62298: f64, t62299: f64, t62300: f64, t62301: f64, t62303: f64, t62304: f64, t62305: f64, t62306: f64) -> f64 {
    let t63160 = t18838 * t2411;
    let t63164 = t4537 * t890;
    let t63170 = 8.0_f64 * t14436 * t1940 * t63164 - 2.0_f64 * t1940 * t63160 * t890 + 12.0_f64 * t18256 * t50080 + t62297 + t62298 - t62299 + t62300 + t62301 + t62303 + t62304 + t62305 + t62306;
    t63170
}
