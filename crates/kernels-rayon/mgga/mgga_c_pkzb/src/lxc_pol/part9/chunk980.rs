//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 980/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk980(t179: f64, t7350: f64, t780: f64, t1066: f64, t5672: f64, t299: f64, t2939: f64, t771: f64, t2068: f64, t2739: f64, t655: f64, t759: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7751 = t179 * t780 * t7350;
    let t7755 = t179 * t5672 * t1066;
    let t7756 = t299 * t7755;
    let t7760 = t771 * t2939;
    let t7765 = t179 * t2068 * t2739;
    let t7767 = 0.57165357490759649296e-3_f64 * t299 * t7765;
    let t7768 = t759 * t655;
    (t7751, t7755, t7756, t7760, t7765, t7767, t7768)
}
