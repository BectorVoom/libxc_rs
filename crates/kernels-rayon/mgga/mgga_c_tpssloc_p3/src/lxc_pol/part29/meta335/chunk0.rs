//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1395/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1395(t1011: f64, t11812: f64, t1212: f64, t486: f64, t676: f64, t1216: f64, t248: f64, t1213: f64, t1226: f64, t3566: f64, t11552: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11813 = t11812 * t1011;
    let t11814 = t11813 * t1212;
    let t11818 = t676 * t486;
    let t11820 = t248 * t11818 * t1216;
    let t11821 = t1213 * t11820;
    let t11825 = t3566 * t1226;
    let t11832 = t221 * t11552;
    (t11813, t11814, t11818, t11820, t11821, t11825, t11832)
}
