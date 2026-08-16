//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2081/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2081(t10073: f64, t1579: f64, t1958: f64, t25390: f64, t25305: f64, t99380: f64, t213: f64, t27265: f64, t2453: f64, t2458: f64, t7760: f64, t25331: f64, t27213: f64) -> (f64, f64, f64, f64, f64) {
    let t99423 = t10073 * t25390 * t1958 * t1579;
    let t99425 = t25305 * t99380;
    let t99429 = t213 * t27265;
    let t99435 = t2453 * t7760 * t2458;
    let t99456 = t27213 * t25331;
    (t99423, t99425, t99429, t99435, t99456)
}
