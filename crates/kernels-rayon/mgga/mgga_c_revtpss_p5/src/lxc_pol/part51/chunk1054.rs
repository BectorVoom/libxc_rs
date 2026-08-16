//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1054/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1054(t31999: f64, t8513: f64, t93488: f64, t1982: f64, t31926: f64, t3268: f64, t31927: f64, t994: f64, t120361: f64, t11921: f64, t247: f64, t31920: f64, t31921: f64) -> (f64, f64, f64, f64, f64) {
    let t120495 = t8513 * t93488 * t31999;
    let t120507 = t1982 * t31926 * t3268;
    let t120513 = t994 * t31927;
    let t120532 = t994 * t120361;
    let t120538 = t31920 * t247 * t11921 * t31921;
    (t120495, t120507, t120513, t120532, t120538)
}
