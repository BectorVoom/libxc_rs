//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1072/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1072(t1352: f64, t31211: f64, t31181: f64, t553: f64, t1332: f64, t1336: f64, t31192: f64, t31197: f64, t31200: f64, t31205: f64, t31209: f64, t544: f64, t8483: f64) -> (f64, f64, f64) {
    let t31212 = t31211 * t1352;
    let t31214 = t553 * t31181;
    let t31216 = t1332 * t8483 - t1336 * t31212 + t31214 * t544 - t31192 - t31197 - t31200 - t31205 + t31209;
    (t31212, t31214, t31216)
}
