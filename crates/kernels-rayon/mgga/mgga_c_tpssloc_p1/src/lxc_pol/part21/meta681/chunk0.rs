//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2491/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2491(t41115: f64, t4250: f64, t4166: f64, t9637: f64, t2649: f64, t13257: f64, t2617: f64, t4184: f64, t4257: f64, t9993: f64, t13176: f64, t2638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46649 = t41115 * t4250;
    let t46657 = t4166 * t9637;
    let t46658 = t46657 * t2649;
    let t46661 = t2617 * t13257 * t4184;
    let t46663 = t9993 * t4257;
    let t46667 = t13176 * t2638;
    (t46649, t46657, t46658, t46661, t46663, t46667)
}
