//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 740/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk740<F: Float>(t31590: F, t6508: F, t197: F, t3338: F, t161: F, t10215: F, t158: F, t475: F, t2787: F, t6509: F, t599: F, t123: F, t25760: F, t426: F, t1352: F, t3339: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31591 = t6508 * t31590;
    let t31730 = t197 * t3338;
    let t31731 = t31730 * t161;
    let t31740 = t158 * t10215;
    let t31747 = t3338 * t475;
    let t31748 = t6508 * t31747;
    let t31769 = t2787 * t6509;
    let t31828 = t599 * t10215;
    let t31903 = t25760 * t123;
    let t32005 = t31590 * t426;
    let t32067 = t3339 * t1352;
    (t31591, t31730, t31731, t31740, t31747, t31748, t31769, t31828, t31903, t32005, t32067)
}
