//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1461/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1461(t33211: f64, t7056: f64, t122660: f64, t2039: f64, t26135: f64, t88: f64, t33596: f64, t31537: f64, t7801: f64, t31717: f64, t27170: f64, t8601: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t122720 = t33211 * t7056;
    let t122721 = t122660 * t2039;
    let t122722 = t88 * t26135;
    let t122723 = t122722 * t2039;
    let t122724 = t33596 * t7056;
    let t122725 = t31537 * t7801;
    let t122726 = t31717 * t7801;
    let t122727 = t8601 * t27170;
    (t122720, t122721, t122723, t122724, t122725, t122726, t122727)
}
