//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1169/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1169(t26179: f64, t29548: f64, t29554: f64, t7349: f64, t28640: f64, t7709: f64, t29562: f64, t95319: f64, t108978: f64, t2047: f64, t108986: f64, t116: f64, t30552: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110016 = t26179 * t29548;
    let t110018 = t29554 * t7349;
    let t110020 = t7709 * t28640;
    let t110022 = t95319 * t29562;
    let t110039 = t2047 * t108978;
    let t110044 = t2047 * t108986;
    let t110110 = t30552 * t116;
    (t110016, t110018, t110020, t110022, t110039, t110044, t110110)
}
