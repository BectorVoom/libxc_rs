//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1840/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1840(t25410: f64, t93320: f64, t7063: f64, t860: f64, t25374: f64, t11007: f64, t1955: f64, t7056: f64, t93189: f64, t93169: f64, t1113: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93321 = t93320 * t25410;
    let t93341 = t7063 * t860;
    let t93342 = t93341 * t25374;
    let t93349 = t1955 * t7056 * t11007;
    let t93364 = t93320 * t25374;
    let t93371 = t93189 * t25410;
    let t93374 = t93341 * t25410;
    let t93377 = t93169 * t25374;
    let t94245 = t2411 * t1113;
    (t93321, t93342, t93349, t93364, t93371, t93374, t93377, t94245)
}
