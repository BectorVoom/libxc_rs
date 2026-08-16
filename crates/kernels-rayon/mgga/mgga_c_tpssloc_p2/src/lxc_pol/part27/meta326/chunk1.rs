//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1404/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1404(t11797: f64, t1227: f64, t248: f64, t3248: f64, t3521: f64, t1009: f64, t3481: f64, t1011: f64, t1212: f64, t486: f64, t676: f64, t1216: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11798 = t1227 * t11797;
    let t11801 = t248 * t3521 * t3248;
    let t11802 = t1227 * t11801;
    let t11812 = t3481 * t1009;
    let t11813 = t11812 * t1011;
    let t11814 = t11813 * t1212;
    let t11818 = t676 * t486;
    let t11820 = t248 * t11818 * t1216;
    (t11798, t11802, t11812, t11814, t11818, t11820)
}
