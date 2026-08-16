//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 993/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk993(t121: f64, t3584: f64, t248: f64, t3243: f64, t1227: f64, t1229: f64, t676: f64, t1090: f64, t3536: f64, t3572: f64, t3252: f64, t3521: f64) -> (f64, f64, f64, f64, f64) {
    let t11784 = t121 * t3584;
    let t11786 = t248 * t11784 * t3243;
    let t11787 = t1227 * t11786;
    let t11789 = t676 * t1229;
    let t11791 = t248 * t11789 * t1090;
    let t11792 = t1227 * t11791;
    let t11794 = t3536 * t3572;
    let t11797 = t248 * t3521 * t3252;
    (t11787, t11789, t11792, t11794, t11797)
}
