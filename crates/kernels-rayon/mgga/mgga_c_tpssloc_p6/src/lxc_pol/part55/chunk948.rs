//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 948/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk948(t1410: f64, t2240: f64, t4017: f64, t71: f64, t12568: f64, t33: f64, t3953: f64, t608: f64, t1437: f64, t641: f64, t72: f64, t4021: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26016 = t2240 * t1410;
    let t26024 = t71 * t4017;
    let t26028 = t12568 * t33;
    let t26055 = t3953 * t608;
    let t26062 = t641 * t1437;
    let t26063 = t72 * t26062;
    let t26066 = t79 * t4021;
    (t26016, t26024, t26028, t26055, t26063, t26066)
}
