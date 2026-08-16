//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3288/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3288(t40076: f64, t40079: f64, t40194: f64, t40198: f64, t62290: f64, t62293: f64, t62296: f64, t62297: f64, t62298: f64, t62299: f64, t62300: f64, t62301: f64, t62303: f64, t62304: f64, t62305: f64, t62306: f64, t62307: f64, t62308: f64, t62311: f64, t62312: f64) -> f64 {
    let t62313 = t62290 + t62293 + t62296 + t62297 + t62298 - t62299 + t62300 + t62301 + t62303 + t62304 + t62305 + t62306 + t40076 - t40079 + t40194 + t40198 + t62307 - t62308 + t62311 - t62312;
    t62313
}
