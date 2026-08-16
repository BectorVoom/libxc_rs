//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1339/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1339(t10868: f64, t820: f64, t843: f64, t2482: f64, t27: f64, t823: f64, t9948: f64, t2681: f64, t2719: f64, t10111: f64, t9720: f64, t2237: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40348 = t820 * t10868 * t843;
    let t40352 = t2482 * t10868 * t27;
    let t40360 = t820 * t823 * t9948;
    let t40398 = t820 * t2719 * t2681;
    let t40406 = t10111 * t823 * t9720;
    let t40424 = t2482 * t823 * t2237;
    (t40348, t40352, t40360, t40398, t40406, t40424)
}
