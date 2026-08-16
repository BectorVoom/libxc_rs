//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 703/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk703(t10703: f64, t29215: f64, t15299: f64, t28516: f64, t4260: f64, t6334: f64, t15229: f64, t28520: f64, t15290: f64, t28524: f64, t1882: f64, t7042: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29216 = t10703 * t29215;
    let t29219 = t15299 * t28516;
    let t29222 = t6334 * t4260;
    let t29223 = t10703 * t29222;
    let t29226 = t15229 * t28520;
    let t29229 = t15290 * t28524;
    let t29232 = t1882 * t7042;
    (t29216, t29219, t29222, t29223, t29226, t29229, t29232)
}
