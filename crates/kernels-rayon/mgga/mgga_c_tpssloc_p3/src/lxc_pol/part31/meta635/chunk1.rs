//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1900/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1900(t22633: f64, t22635: f64, t26337: f64, t5353: f64, t5325: f64, t90488: f64, t1307: f64, t567: f64, t6330: f64, t90591: f64, t28199: f64, t6897: f64, t794: f64) -> (f64, f64, f64, f64) {
    let t97577 = t22633 * t22635 * t26337 * t5353;
    let t97583 = t22633 * t22635 * t90488 * t5325;
    let t97588 = t90591 * t22635 * t567 * t6330 * t1307;
    let t97599 = t6897 * t794 * t28199;
    (t97577, t97583, t97588, t97599)
}
