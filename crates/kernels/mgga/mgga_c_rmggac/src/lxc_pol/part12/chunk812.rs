//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 812/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk812<F: Float>(t7404: F, t8571: F, t1635: F, t1971: F, t495: F, t7230: F, t880: F, t236: F, t3351: F, t5204: F, t9188: F, t3352: F, t511: F, t5211: F, t2004: F, t38472: F) -> (F, F, F, F, F) {
    let t39735 = t8571 * t7404;
    let t39742 = t7230 * t1971 * t880 * t1635 * t495;
    let t39748 = t3351 * t9188 * t236 * t5204;
    let t39752 = t3351 * t3352 * t511 * t5211;
    let t39754 = t38472 * t2004;
    (t39735, t39742, t39748, t39752, t39754)
}
