//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3039/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3039(t14574: f64, t2439: f64, t2777: f64, t40297: f64, t4500: f64, t10069: f64, t14504: f64, t4423: f64, t860: f64, t1558: f64, t2760: f64, t14557: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51355 = t2439 * t2777 * t14574;
    let t51371 = t40297 * t4500;
    let t51373 = t10069 * t14504;
    let t51375 = t860 * t4423;
    let t51380 = t2760 * t1558;
    let t51390 = t9303 * t14557;
    (t51355, t51371, t51373, t51375, t51380, t51390)
}
