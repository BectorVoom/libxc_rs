//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 791/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk791(t38569: f64, t7192: f64, t7335: f64, t8355: f64, t7345: f64, t2185: f64, t9221: f64, t1997: f64, t8450: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38570 = t7192 * t38569;
    let t38608 = t7335 * t8355;
    let t38610 = t7345 * t8355;
    let t38621 = t9221 * t2185;
    let t38622 = t38621 * t1997;
    let t38623 = 0.24829349937757072982e-4_f64 * t38622;
    let t38638 = t8450 * t2185;
    (t38570, t38608, t38610, t38621, t38623, t38638)
}
