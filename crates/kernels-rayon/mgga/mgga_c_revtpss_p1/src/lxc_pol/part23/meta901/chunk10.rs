//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2877/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2877(t23421: f64, t2411: f64, t1940: f64, t23429: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t41154: f64, t76955: f64, t76957: f64, t76960: f64, t890: f64) -> f64 {
    let t77357 = t23421 * t2411;
    let t77360 = -6.0_f64 * t1940 * t23429 * t41154 * t890 - t1940 * t77357 * t890 + t39520 - t39528 + t39531 + t39534 + t39537 - t39540 + t76955 + t76957 + t76960;
    t77360
}
