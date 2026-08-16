//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1908/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1908(t28083: f64, t28106: f64, t539: f64, t2015: f64, t6460: f64, t3887: f64, t1842: f64, t26337: f64, t22635: f64, t22633: f64, t1825: f64, t26421: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28107 = t28083 + t28106;
    let t28108 = t539 * t28107;
    let t28110 = t2015 * t6460;
    let t28111 = t3887 * t28110;
    let t28116 = t26337 * t1842;
    let t28117 = t22635 * t28116;
    let t28118 = t22633 * t28117;
    let t28130 = t26421 * t1825;
    (t28107, t28108, t28111, t28116, t28117, t28118, t28130)
}
