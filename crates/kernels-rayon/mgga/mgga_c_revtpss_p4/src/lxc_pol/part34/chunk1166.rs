//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1166/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1166(t265: f64, t393: f64, t1102: f64, t1699: f64, t198: f64, t25713: f64, t27712: f64, t29894: f64, t29930: f64, t336: f64, t5023: f64, t6396: f64, t6400: f64, t7181: f64) -> f64 {
    let t394 = t265 < t393;
    let t29931 = piecewise3(t394, t1102 * t198 * t29894 * t336 - 2.0_f64 * t1699 * t27712 * t5023 + 2.0_f64 * t25713 * t5023 * t6400 - t5023 * t6396 * t7181, t29930);
    t29931
}
