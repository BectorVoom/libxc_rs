//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 841/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk841<F: Float>(t12830: F, t1422: F, t3533: F, t1365: F, t3619: F, t5953: F, t1056: F, t1390: F, t3283: F) -> (F, F, F, F) {
    let t12860 = t1422 * t3533 * t12830;
    let t12863 = t1365 * t3619;
    let t12864 = t5953 * t12863;
    let t12867 = t1390 * t1056;
    let t12868 = t12867 * t3283;
    (t12860, t12863, t12864, t12868)
}
