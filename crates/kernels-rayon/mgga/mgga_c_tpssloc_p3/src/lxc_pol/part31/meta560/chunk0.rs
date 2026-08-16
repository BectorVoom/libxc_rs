//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1789/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1789(t23168: f64, t25338: f64, t23012: f64, t7485: f64, t25046: f64, t6579: f64, t1484: f64, t2717: f64, t82099: f64, t7489: f64, t82120: f64, t23164: f64, t23204: f64, t25341: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t86950 = t23168 * t25338;
    let t86955 = t23012 * t7485;
    let t86967 = t6579 * t25046;
    let t86969 = t2717 * t1484;
    let t86983 = 0.52089578783527170489e-1_f64 * t82099;
    let t86991 = t23012 * t7489;
    let t86994 = 0.3289868133696452873e-1_f64 * t82120;
    let t87028 = t23164 * t23204 * t25341;
    (t86950, t86955, t86967, t86969, t86983, t86991, t86994, t87028)
}
