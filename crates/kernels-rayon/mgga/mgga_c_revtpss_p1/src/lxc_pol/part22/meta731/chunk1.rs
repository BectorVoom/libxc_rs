//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2789/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2789(t40452: f64, t685: f64, t775: f64, t855: f64, t242: f64, t240: f64, t72: f64, t10710: f64, t9775: f64, t10733: f64, t10716: f64, t10741: f64) -> (f64, f64, f64, f64, f64) {
    let t40455 = t40452 * t855 * t685 * t775;
    let t40459 = t242 * t242;
    let t40460 = 1.0_f64 / t40459;
    let t40462 = t240 * t40460 * t72;
    let t40473 = t9775 * t10710;
    let t40475 = t9775 * t10733;
    let t40477 = t10716 * t10741;
    (t40455, t40462, t40473, t40475, t40477)
}
