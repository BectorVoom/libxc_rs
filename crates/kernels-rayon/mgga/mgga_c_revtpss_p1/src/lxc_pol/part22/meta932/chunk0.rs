//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3161/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3161(t13014: f64, t5373: f64, t12998: f64, t1222: f64, t140: f64, t17404: f64, t12941: f64, t5293: f64, t5274: f64, t1263: f64, t16750: f64, t17547: f64, t3704: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57290 = t5373 * t13014;
    let t57292 = t5373 * t12998;
    let t57295 = t1222 * t140 * t17404;
    let t57297 = t5293 * t12941;
    let t57299 = t5274 * t12941;
    let t57303 = t1263 * t16750;
    let t57314 = t17547 * t3704;
    (t57290, t57292, t57295, t57297, t57299, t57303, t57314)
}
