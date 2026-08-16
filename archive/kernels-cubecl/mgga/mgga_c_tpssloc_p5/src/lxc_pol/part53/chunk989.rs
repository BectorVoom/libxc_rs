//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 989/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk989<F: Float>(t1484: F, t2047: F, t22986: F, t6646: F, t829: F, t22893: F, t23164: F, t33375: F, t33383: F, t6562: F, t794: F, t234: F, t7823: F) -> (F, F, F, F, F) {
    let t121495 = t2047 * t1484;
    let t121498 = t22986 * t6646 * t121495 * t829;
    let t121501 = t23164 * t22893 * t33375;
    let t121504 = t6562 * t794 * t33383;
    let t121506 = t234 * t7823;
    (t121495, t121498, t121501, t121504, t121506)
}
