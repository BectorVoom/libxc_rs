//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 272/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk272<F: Float>(t386: F, t90: F, t71: F, t64: F, t397: F, t110: F, t19: F, t67: F, t20: F, t5: F, t163: F, t1: F, t341: F, t394: F, t343: F, t97: F) -> (F, F, F, F, F, F, F, F) {
    let t1088 = t90 * t386;
    let t1091 = t71 * t71;
    let t1092 = 1.0 / t1091;
    let t1093 = t64 * t1092;
    let t1094 = t397 * t397;
    let t1097 = 1.0 / t110;
    let t1099 = t1097 * t67 * t19;
    let t1100 = t20 * t5;
    let t1101 = t1100 * t163;
    let t1105 = t341 * t394 * t1;
    let t1108 = t343 * t97;
    (t1088, t1093, t1094, t1097, t1099, t1101, t1105, t1108)
}
