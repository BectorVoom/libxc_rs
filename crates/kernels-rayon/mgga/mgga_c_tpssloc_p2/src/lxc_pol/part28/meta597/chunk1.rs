//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1896/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1896(t1437: f64, t2303: f64, t72: f64, t4021: f64, t641: f64, t645: f64, t7445: f64, t12619: f64, t71: f64, t1433: f64, t2307: f64, t12719: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90227 = t72 * t2303 * t1437;
    let t90232 = t72 * t641 * t4021;
    let t90247 = t7445 * t645;
    let t90257 = t71 * t12619;
    let t90297 = t72 * t1433 * t2307;
    let t90334 = t72 * t79 * t12719;
    (t90227, t90232, t90247, t90257, t90297, t90334)
}
