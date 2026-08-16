//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 522/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk522(t339: f64, t39: f64, t11: f64, t340: f64, t14: f64, t1526: f64, t4906: f64, t9483: f64, t10915: f64, t240: f64, t3691: f64, t2917: f64) -> (f64, f64, f64, f64) {
    let t15564 = t339 * t39;
    let t15565 = t340 * t11;
    let t15567 = t15564 * t15565 * t14;
    let t17685 = t1526 * t9483 * t4906;
    let t17687 = t10915 * t240;
    let t17688 = t17687 * t3691;
    let t17694 = t2917 * t240;
    (t15567, t17685, t17688, t17694)
}
