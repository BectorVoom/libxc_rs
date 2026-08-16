//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1144/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1144(t214: f64, t5631: f64, t2717: f64, t5636: f64, t258: f64, t5544: f64, t28267: f64, t81651: f64, t82074: f64, t5527: f64, t857: f64, t23204: f64, t28298: f64, t81640: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98133 = t214 * t5631;
    let t98161 = t2717 * t5636;
    let t98169 = t258 * t5544;
    let t98213 = t81651 * t82074 * t28267;
    let t98224 = t857 * t5527;
    let t98237 = t81640 * t23204 * t28298;
    (t98133, t98161, t98169, t98213, t98224, t98237)
}
