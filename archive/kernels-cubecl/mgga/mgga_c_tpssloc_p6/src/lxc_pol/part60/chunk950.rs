//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 950/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk950<F: Float>(t114790: F, t23164: F, t7479: F, t23168: F, t33419: F, t33395: F, t814: F, t1484: F, t2047: F, t22893: F, t33375: F, t33383: F, t6562: F, t794: F) -> (F, F, F, F, F, F) {
    let t121464 = t23164 * t114790 * t7479;
    let t121469 = t23168 * t33419;
    let t121488 = t814 * t33395;
    let t121495 = t2047 * t1484;
    let t121501 = t23164 * t22893 * t33375;
    let t121504 = t6562 * t794 * t33383;
    (t121464, t121469, t121488, t121495, t121501, t121504)
}
