//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 781/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk781(t3576: f64, t5064: f64, t1725: f64, t698: f64, t1174: f64, t5168: f64, t588: f64, t592: f64, t2528: f64, t5154: f64, t2535: f64, t118: f64, t1787: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15740 = t5064 * t3576;
    let t15753 = t698 * t1725;
    let t15754 = t1174 * t15753;
    let t15875 = t588 * t5168;
    let t15877 = t592 * t5168;
    let t15890 = t5154 * t2528;
    let t15895 = t5154 * t2535;
    let t15908 = t1787 * t118;
    (t15740, t15754, t15875, t15877, t15890, t15895, t15908)
}
