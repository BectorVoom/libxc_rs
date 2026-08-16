//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1678/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1678(t23168: f64, t7480: f64, t6547: f64, t7489: f64, t23237: f64, t7488: f64, t1880: f64, t4300: f64, t6571: f64, t6553: f64, t1519: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25209 = t23168 * t7480;
    let t25211 = t6547 * t7489;
    let t25213 = t23237 * t7488;
    let t25214 = t1880 * t25213;
    let t25216 = t6571 * t4300;
    let t25217 = t6553 * t25216;
    let t25218 = t1880 * t25217;
    let t25224 = t214 * t1519;
    (t25209, t25211, t25213, t25214, t25216, t25217, t25218, t25224)
}
