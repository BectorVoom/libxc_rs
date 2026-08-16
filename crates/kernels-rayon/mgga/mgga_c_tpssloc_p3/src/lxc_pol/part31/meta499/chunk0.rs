//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1695/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1695(t1339: f64, t28100: f64, t22827: f64, t22833: f64, t6396: f64, t1842: f64, t26337: f64, t22635: f64, t22633: f64, t1825: f64, t26421: f64, t6976: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28101 = t1339 * t28100;
    let t28102 = t22827 * t28101;
    let t28104 = t22833 * t6396;
    let t28116 = t26337 * t1842;
    let t28117 = t22635 * t28116;
    let t28118 = t22633 * t28117;
    let t28130 = t26421 * t1825;
    let t28131 = t6976 * t28130;
    (t28101, t28102, t28104, t28116, t28117, t28118, t28130, t28131)
}
