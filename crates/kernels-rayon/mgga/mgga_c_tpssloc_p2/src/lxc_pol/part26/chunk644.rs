//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 644/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk644(t210: f64, t214: f64, t3734: f64, t1314: f64, t792: f64, t118: f64, t1307: f64, t794: f64, t3719: f64, t116: f64, t534: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3736 = t210 * t214 * t3734;
    let t3739 = t792 * t1314;
    let t3741 = t118 * t794 * t1307;
    let t3742 = t3739 * t3741;
    let t3745 = t210 * t214 * t3719;
    let t3748 = t534 * t116;
    let t3749 = t3748 * t212;
    (t3736, t3739, t3741, t3742, t3745, t3748, t3749)
}
