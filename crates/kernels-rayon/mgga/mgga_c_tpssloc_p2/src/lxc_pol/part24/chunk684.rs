//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 684/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk684(t248: f64, t2691: f64, t557: f64, t555: f64, t1361: f64, t835: f64, t1336: f64, t1369: f64, t1995: f64, t241: f64, t67: f64, t3734: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3862 = t2691 * t557 * t248;
    let t3864 = 119.0_f64 / 13824.0_f64 * t555 * t3862;
    let t3865 = t1361 * t835;
    let t3866 = t1336 * t3865;
    let t3867 = t3866 * t1369;
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    let t3872 = t3870 * t820 * t3734;
    (t3862, t3864, t3865, t3866, t3867, t3870, t3872)
}
