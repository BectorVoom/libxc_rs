//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 694/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk694(t3787: f64, t59: f64, t240: f64, t1336: f64, t6943: f64, t835: f64, t6604: f64, t6919: f64, t6950: f64, t6597: f64, t6924: f64, t281: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22759 = t3787 * t59;
    let t22760 = t22759 * t240;
    let t22761 = t1336 * t22760;
    let t22764 = t6943 * t835;
    let t22765 = t1336 * t22764;
    let t22779 = t6919 * t6604;
    let t22782 = t6950 * t835;
    let t22783 = t1336 * t22782;
    let t22791 = t6597 * t6924;
    let t22792 = t22791 * t281;
    (t22759, t22761, t22765, t22779, t22783, t22792)
}
