//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2999/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2999<F: Float>(t19882: F, t4834: F, t1062: F, t23960: F, t11921: F, t23964: F, t247: F, t4837: F, t11246: F, t23833: F, t3172: F, t1063: F, t23851: F) -> (F, F, F, F, F) {
    let t79553 = t4834 * t19882;
    let t79559 = t23960 * t1062;
    let t79564 = t4837 * t247 * t11921 * t23964;
    let t79575 = t11246 * t3172 * t23833;
    let t79580 = t1063 * t3172 * t23851;
    (t79553, t79559, t79564, t79575, t79580)
}
