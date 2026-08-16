//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2070/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2070(t10073: f64, t25403: f64, t27198: f64, t1955: f64, t99270: f64, t2471: f64, t27202: f64, t15003: f64, t93194: f64, t27266: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t99297 = t10073 * t27198 * t25403;
    let t99303 = t1955 * t99270;
    let t99307 = t27202 * t2471;
    let t99313 = t93194 * t15003;
    let t99321 = t27266 * t72 * t686;
    (t99297, t99303, t99307, t99313, t99321)
}
