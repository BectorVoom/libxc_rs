//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 960/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk960(t17612: f64, t275: f64, t176: f64, t1006: f64, t5471: f64, t1584: f64, t1567: f64, t2325: f64, t5242: f64, t1442: f64, t15067: f64, t15066: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17613 = t17612 * t275;
    let t17615 = t176 * t17613 * sigma2;
    let t17618 = t1006 * t5471;
    let t17619 = t17618 * t1584;
    let t17622 = t2325 * t1567;
    let t17623 = t17622 * t5242;
    let t17626 = t15067 * t1442;
    let t17627 = t15066 * t17626;
    (t17615, t17618, t17619, t17622, t17623, t17627)
}
