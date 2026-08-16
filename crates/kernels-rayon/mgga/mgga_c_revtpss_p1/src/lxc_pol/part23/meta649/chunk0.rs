//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2374/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2374(t10868: f64, t2482: f64, t27: f64, t820: f64, t823: f64, t9948: f64, t839: f64, t2681: f64, t2719: f64, t10111: f64, t9720: f64, t685: f64, t827: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40352 = t2482 * t10868 * t27;
    let t40360 = t820 * t823 * t9948;
    let t40361 = t40360 * t839;
    let t40398 = t820 * t2719 * t2681;
    let t40406 = t10111 * t823 * t9720;
    let t40409 = t40406 * t827 * t685 * t837;
    (t40352, t40360, t40361, t40398, t40406, t40409)
}
