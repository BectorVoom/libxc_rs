//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1880/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1880(t19889: f64, t91004: f64, t91006: f64, t28182: f64, t6914: f64, t19660: f64, t22633: f64, t3807: f64, t6976: f64, t22685: f64, t22881: f64, t6330: f64, t6637: f64) -> (f64, f64, f64, f64) {
    let t97146 = t91004 * t91006 * t19889;
    let t97148 = t6914 * t28182;
    let t97152 = t22633 * t6976 * t19660 * t3807;
    let t97158 = t22685 * t6637 * t22881 * t6330;
    (t97146, t97148, t97152, t97158)
}
