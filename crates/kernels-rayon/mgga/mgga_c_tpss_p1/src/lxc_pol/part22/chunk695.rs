//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 695/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk695(t1224: f64, t3332: f64, t774: f64, t2377: f64, t242: f64, t527: f64, t525: f64, t1242: f64, t339: f64, t789: f64) -> (f64, f64, f64, f64) {
    let t3334 = t1224 * t774 * t3332;
    let t3338 = t2377 * t527 * t242;
    let t3340 = 119.0_f64 / 13824.0_f64 * t525 * t3338;
    let t3342 = t339 * t1242 * t789;
    (t3334, t3338, t3340, t3342)
}
