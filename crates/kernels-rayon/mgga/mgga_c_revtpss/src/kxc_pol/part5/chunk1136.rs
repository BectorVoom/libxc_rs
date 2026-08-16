//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1136/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1136(t1260: f64, t5261: f64, t3647: f64, t5378: f64, t247: f64, t3634: f64, t5056: f64, t1261: f64, t12916: f64, t5334: f64, t5331: f64, t1778: f64, t3682: f64) -> (f64, f64, f64, f64, f64) {
    let t17763 = t5261 * t1260;
    let t17767 = 0.19055119163586549765e-3_f64 * t3647 * t5378;
    let t17769 = t247 * t3634 * t5056;
    let t17771 = 0.19055119163586549765e-3_f64 * t1261 * t17769;
    let t17789 = t12916 * t5334;
    let t17791 = 0.28582678745379824648e-3_f64 * t5331 * t17789;
    let t17792 = t1778 * t3682;
    (t17763, t17767, t17771, t17791, t17792)
}
