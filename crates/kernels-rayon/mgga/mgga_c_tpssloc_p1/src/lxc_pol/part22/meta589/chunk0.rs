//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2102/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2102(t46573: f64, t1516: f64, t40965: f64, t242: f64, t812: f64, t841: f64, t41115: f64, t4250: f64, t4166: f64, t9637: f64, t13176: f64, t2638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46574 = 119.0_f64 / 1152.0_f64 * t46573;
    let t46577 = t40965 * t1516;
    let t46628 = t812 * t841 * t242;
    let t46649 = t41115 * t4250;
    let t46650 = 119.0_f64 / 1152.0_f64 * t46649;
    let t46657 = t4166 * t9637;
    let t46667 = t13176 * t2638;
    (t46574, t46577, t46628, t46650, t46657, t46667)
}
