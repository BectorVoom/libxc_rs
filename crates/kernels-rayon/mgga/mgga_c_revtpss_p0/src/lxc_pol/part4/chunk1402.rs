//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1402/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1402(t16756: f64, t5333: f64, t3720: f64, t3588: f64, t471: f64, t5332: f64, t12916: f64, t5334: f64, t5331: f64, t1778: f64, t3682: f64, t1774: f64, t3617: f64) -> (f64, f64, f64, f64, f64) {
    let t17780 = t16756 * t5333;
    let t17781 = t3720 * t17780;
    let t17784 = t3588 * t471;
    let t17785 = t5332 * t17784;
    let t17786 = t3720 * t17785;
    let t17789 = t12916 * t5334;
    let t17791 = 0.28582678745379824648e-3_f64 * t5331 * t17789;
    let t17792 = t1778 * t3682;
    let t17794 = t3617 * t1774;
    (t17781, t17786, t17791, t17792, t17794)
}
