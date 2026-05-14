//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 745/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk745<F: Float>(t12588: F, t12589: F, t12592: F, t12595: F, t12601: F, t12604: F, t12608: F, t12614: F, t12620: F, t12624: F, t12626: F, t2895: F, t834: F, t839: F, t12585: F) -> (F,) {
    let t12629 = t12588 - 0.21687161765563048428e-1 * t2895 * t12589 + 0.16265371324172286321e-1 * t2895 * t12592 + 0.48159446095139119799e0 * t2895 * t12595 + t12601 - t12604 - t12608 - 0.1025389702100779493e4 * t839 * t12614 + t12620 + t12624 - 0.56969282336565386482e-3 * t834 * t12626;
    let t12630 = t12585 + t12629;
    (t12630,)
}
