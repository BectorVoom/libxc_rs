//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2189/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2189(t25375: f64, t99365: f64, t1579: f64, t25392: f64, t4481: f64, t92921: f64, t10073: f64, t1958: f64, t25390: f64, t25305: f64, t99380: f64, t213: f64, t27265: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99412 = t25375 * t99365;
    let t99414 = t25392 * t1579;
    let t99420 = 0.19514881078765566038e-1_f64 * t92921 * t4481;
    let t99423 = t10073 * t25390 * t1958 * t1579;
    let t99425 = t25305 * t99380;
    let t99429 = t213 * t27265;
    (t99412, t99414, t99420, t99423, t99425, t99429)
}
