//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1192/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1192(t120042: f64, t1549: f64, t31827: f64, t31831: f64, t31755: f64, t31756: f64, t4364: f64, t4424: f64, t125984: f64, t25759: f64, t126030: f64, t1113: f64, t7782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126396 = t120042 * t1549;
    let t126397 = t31827 * t126396;
    let t126399 = t31831 * t126396;
    let t126403 = t31755 * t4364 * t31756 * t4424;
    let t127193 = t25759 * t125984;
    let t127199 = t25759 * t126030;
    let t127207 = t1113 * t7782;
    (t126397, t126399, t126403, t127193, t127199, t127207)
}
