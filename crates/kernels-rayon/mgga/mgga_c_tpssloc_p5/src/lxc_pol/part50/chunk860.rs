//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 860/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk860(t22751: f64, t6970: f64, t3777: f64, t6944: f64, t3787: f64, t59: f64, t6943: f64, t835: f64, t1336: f64, t1354: f64, t6604: f64, t6919: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22752 = t22751 * t6970;
    let t22756 = t3777 * t6944;
    let t22759 = t3787 * t59;
    let t22764 = t6943 * t835;
    let t22765 = t1336 * t22764;
    let t22766 = t22765 * t1354;
    let t22779 = t6919 * t6604;
    (t22752, t22756, t22759, t22765, t22766, t22779)
}
