//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2057/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2057(t1032: f64, t3727: f64, t2148: f64, t1276: f64, t3140: f64, t26894: f64, t26921: f64, t1294: f64, t471: f64, t355: f64, t1204: f64, t7627: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96873 = t3727 * t1032;
    let t96874 = t2148 * t96873;
    let t96910 = t2148 * t3727 * t3140 * t1276;
    let t96927 = t26894 * t26921;
    let t96928 = t471 * t1294;
    let t96929 = t355 * t96928;
    let t96933 = t1204 * t7627;
    (t96873, t96874, t96910, t96927, t96928, t96929, t96933)
}
