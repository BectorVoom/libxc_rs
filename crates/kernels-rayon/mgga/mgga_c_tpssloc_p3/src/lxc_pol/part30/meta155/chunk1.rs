//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 815/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk815(t157: f64, t2658: f64, t1409: f64, t184: f64, t607: f64, t1474: f64, t172: f64, t763: f64, t185: f64, t3966: f64, t707: f64, t1471: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4194 = t2658 * t157;
    let t4195 = t184 * t1409;
    let t4196 = t4195 * t607;
    let t4198 = 12.0_f64 * t4194 * t4196;
    let t4199 = t1474 * t172;
    let t4200 = t4199 * t763;
    let t4201 = 0.5848223622634646207e0_f64 * t4200;
    let t4202 = t185 * t3966;
    let t4204 = 4.0_f64 * t707 * t4202;
    let t4205 = t706 * t1471;
    (t4194, t4195, t4196, t4198, t4199, t4200, t4201, t4202, t4204, t4205)
}
