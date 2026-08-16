//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1230/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1230(t178: f64, t18016: f64, t7707: f64, t7710: f64, t17933: f64, t17930: f64, t1123: f64, t17938: f64, t2030: f64, t5726: f64, t18000: f64, t18002: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21448 = t18016 * t178;
    let t21452 = t7707 * t7710;
    let t21454 = t17933 * t178;
    let t21455 = t17930 * t21454;
    let t21456 = t1123 * t17938;
    let t21457 = t2030 * t5726;
    let t21462 = t18000 * t21454;
    let t21463 = t18002 * t5726;
    (t21448, t21452, t21454, t21455, t21456, t21457, t21462, t21463)
}
