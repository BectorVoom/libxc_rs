//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2198/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2198(t1512: f64, t46667: f64, t16903: f64, t9638: f64, t41008: f64, t5568: f64, t5614: f64, t9674: f64, t16859: f64, t2639: f64, t13360: f64, t4257: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58731 = t46667 * t1512;
    let t58735 = t9638 * t16903;
    let t58744 = t41008 * t5568;
    let t58759 = t9674 * t5614;
    let t58761 = t2639 * t16859;
    let t58763 = t13360 * t4257;
    (t58731, t58735, t58744, t58759, t58761, t58763)
}
