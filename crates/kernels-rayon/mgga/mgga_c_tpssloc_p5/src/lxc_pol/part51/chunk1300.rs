//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1300/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1300(t31820: f64, t576: f64, t1395: f64, t8660: f64, t2029: f64, t7222: f64, t2105: f64, t7002: f64, t2098: f64, t7020: f64, t25: f64, t25353: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t116028 = t576 * t31820;
    let t116032 = t1395 * t8660;
    let t116036 = t7222 * t2029;
    let t116038 = t7002 * t2105;
    let t116044 = t2098 * t7020;
    let t118387 = t25 * t25353;
    (t116028, t116032, t116036, t116038, t116044, t118387)
}
