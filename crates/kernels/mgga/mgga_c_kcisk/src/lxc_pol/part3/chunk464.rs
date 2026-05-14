//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 464/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk464<F: Float>(t1343: F, t3748: F, t1342: F, t3512: F, t1339: F, t1341: F, t3583: F, t1340: F, t1336: F, t140: F, t3529: F, t3575: F, t1299: F, t470: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3749 = t3748 * t1343;
    let t3751 = t3512 * t1342;
    let t3752 = t1339 * t3751;
    let t3754 = t1341 * t3583;
    let t3755 = t1340 * t3754;
    let t3756 = t1339 * t3755;
    let t3759 = t140 * t1336 * t3529;
    let t3760 = t1341 * t3575;
    let t3761 = t1340 * t3760;
    let t3762 = t3759 * t3761;
    let t3764 = t1299 * t470;
    (t3749, t3751, t3752, t3754, t3755, t3756, t3759, t3760, t3761, t3762, t3764)
}
