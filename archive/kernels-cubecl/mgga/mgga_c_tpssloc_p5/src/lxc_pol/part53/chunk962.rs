//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 962/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk962<F: Float>(t112: F, t32281: F, t111: F, t8811: F, t25374: F, t86716: F, t25365: F, t25373: F, t16596: F, t22960: F, t4255: F, t30713: F, t4166: F) -> (F, F, F, F, F, F, F) {
    let t117390 = t32281 * t112;
    let t117397 = t8811 * t111;
    let t118377 = t86716 * t25374;
    let t118407 = t25373 * t25365;
    let t118417 = t25373 * t16596;
    let t118440 = t22960 * t4255;
    let t118532 = t4166 * t30713;
    (t117390, t117397, t118377, t118407, t118417, t118440, t118532)
}
