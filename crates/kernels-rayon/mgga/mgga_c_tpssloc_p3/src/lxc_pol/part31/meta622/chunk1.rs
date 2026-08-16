//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1878/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1878(t28159: f64, t6897: f64, t794: f64, t19763: f64, t1992: f64, t6976: f64, t19739: f64, t22633: f64, t3807: f64, t28131: f64, t81159: f64, t552: f64, t6434: f64) -> (f64, f64, f64, f64, f64) {
    let t97111 = t6897 * t794 * t28159;
    let t97114 = t1992 * t6976 * t19763;
    let t97119 = t22633 * t6976 * t19739 * t3807;
    let t97124 = t81159 * t28131;
    let t97126 = t552 * t6434;
    (t97111, t97114, t97119, t97124, t97126)
}
