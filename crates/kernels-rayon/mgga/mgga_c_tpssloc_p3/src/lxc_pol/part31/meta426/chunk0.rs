//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1551/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1551(t1369: f64, t22783: f64, t3777: f64, t6951: f64, t6597: f64, t6924: f64, t281: f64, t1307: f64, t1361: f64, t22690: f64, t547: f64, t6546: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22784 = t22783 * t1369;
    let t22785 = 7.0_f64 / 288.0_f64 * t22784;
    let t22788 = t3777 * t6951;
    let t22791 = t6597 * t6924;
    let t22792 = t22791 * t281;
    let t22794 = t22690 * t1361 * t1307;
    let t22795 = t22792 * t22794;
    let t22797 = t6546 * t547;
    (t22785, t22788, t22791, t22792, t22794, t22795, t22797)
}
