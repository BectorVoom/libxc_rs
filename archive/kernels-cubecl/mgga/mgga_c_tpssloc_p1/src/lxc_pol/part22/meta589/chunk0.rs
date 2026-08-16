//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2102/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2102<F: Float>(t46573: F, t1516: F, t40965: F, t242: F, t812: F, t841: F, t41115: F, t4250: F, t4166: F, t9637: F, t13176: F, t2638: F) -> (F, F, F, F, F, F) {
    let t46574 = F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t46573;
    let t46577 = t40965 * t1516;
    let t46628 = t812 * t841 * t242;
    let t46649 = t41115 * t4250;
    let t46650 = F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t46649;
    let t46657 = t4166 * t9637;
    let t46667 = t13176 * t2638;
    (t46574, t46577, t46628, t46650, t46657, t46667)
}
