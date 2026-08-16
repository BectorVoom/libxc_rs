//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 984/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk984(t10162: f64, t2439: f64, t1420: f64, t2453: f64, t3908: f64, t1426: f64, t786: f64, t64: f64, t843: f64, t112: f64, t2289: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10163 = t2439 * t10162;
    let t10165 = t2453 * t1420;
    let t10166 = t10165 * t3908;
    let t10174 = t1420 * t1426;
    let t10175 = t786 * t10174;
    let t10199 = t64 * t843;
    let t10201 = 154.0_f64 / 27.0_f64 * t10199 * t112;
    let t10202 = t2289 * t666;
    (t10163, t10166, t10175, t10199, t10201, t10202)
}
