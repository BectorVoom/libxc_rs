//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 755/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk755(t1989: f64, t3336: f64, t207: f64, t7086: f64, t1940: f64, t1963: f64, t198: f64, t2403: f64, t7091: f64, t775: f64, t890: f64, t892: f64) -> (f64, f64) {
    let t7181 = t1989 * t3336;
    let t7188 = t207 * t7086;
    let t7193 = -t1940 * t7091 * t890 + 3.0_f64 * t1963 * t2403 * t775 + t198 * t7188 * t892;
    (t7181, t7193)
}
