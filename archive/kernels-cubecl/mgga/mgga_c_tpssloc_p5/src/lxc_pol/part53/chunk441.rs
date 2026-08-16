//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 441/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk441<F: Float>(t1015: F, t3107: F, t1012: F, t1009: F, t990: F, t1011: F, t1019: F, t1004: F, t1040: F, t1013: F, t361: F, t363: F) -> (F, F, F, F, F, F) {
    let t3108 = t1015 * t3107;
    let t3109 = t1012 * t3108;
    let t3112 = t990 * t1009;
    let t3113 = t3112 * t1011;
    let t3114 = t3113 * t1019;
    let t3117 = t1004 * t1040;
    let t3127 = F::cast_from(1.0_f64) / t1013 / t361;
    let t3128 = t3127 * t363;
    (t3109, t3112, t3114, t3117, t3127, t3128)
}
