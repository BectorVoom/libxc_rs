//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1005/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1005(t10612: f64, t706: f64, t775: f64, t853: f64, t2710: f64, t2793: f64, t9285: f64, t2470: f64, t2804: f64, t874: f64, t875: f64, t9288: f64) -> (f64, f64, f64, f64, f64) {
    let t10613 = t706 * t10612;
    let t10631 = t853 * t775;
    let t10645 = 0.46263278077393568556e-2_f64 * t2710 * t2793 * t9285;
    let t10647 = t874 * t2804 * t2470;
    let t10651 = 0.30356481678079769392e-1_f64 * t874 * t875 * t9288;
    (t10613, t10631, t10645, t10647, t10651)
}
