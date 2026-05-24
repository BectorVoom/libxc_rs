//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1048/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1048<F: Float>(t12566: F, t47: F, t12535: F, t2921: F, t12552: F, t848: F, t247: F, t3327: F, t242: F, t1077: F, t3313: F, t3331: F) -> (F, F, F, F, F, F, F) {
    let t15563 = t47 * t12566;
    let t15564 = t12535 * t2921;
    let t15567 = t12552 * t848;
    let t15570 = t12535 * t848;
    let t15577 = F::new(1.0) / t3327 / t247;
    let t15578 = t242 * t15577;
    let t15579 = t3313 * t1077;
    let t15580 = t15579 * t3331;
    (t15563, t15564, t15567, t15570, t15578, t15579, t15580)
}
