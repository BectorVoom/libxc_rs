//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 403/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk403(t3862: f64, t555: f64, t1361: f64, t835: f64, t1336: f64, t1995: f64, t241: f64, t67: f64, t1376: f64, t566: f64) -> (f64, f64, f64, f64) {
    let t3864 = 119.0_f64 / 13824.0_f64 * t555 * t3862;
    let t3865 = t1361 * t835;
    let t3866 = t1336 * t3865;
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    let t3886 = 1.0_f64 / t1376 / t566;
    (t3864, t3866, t3870, t3886)
}
