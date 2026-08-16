//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1610/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1610(t23237: f64, t7488: f64, t1880: f64, t4300: f64, t6571: f64, t6553: f64, t1519: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25213 = t23237 * t7488;
    let t25214 = t1880 * t25213;
    let t25216 = t6571 * t4300;
    let t25217 = t6553 * t25216;
    let t25218 = t1880 * t25217;
    let t25224 = t214 * t1519;
    (t25213, t25214, t25216, t25217, t25218, t25224)
}
